use models::generated::{notifications, subscriptions, users};
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection, Statement};
use web_push::WebPushClient;

const LOG_KEY: &str = "[Operations::Notifications::Create::ForCommentCreate]: ";

pub async fn execute(
    users: Vec<users::Model>,
    db: DatabaseConnection,
    comment_id: i32,
    isahc_client: web_push::IsahcWebPushClient,
) -> Result<i32, DbErr> {
    let user_iter = users.into_iter();

    let last_inserted_notification =
        match notifications::Entity::insert_many(user_iter.clone().map(|user| {
            notifications::ActiveModel {
                user_id: Set(user.id),
                notification_type: Set(1),
                seen: Set(false),
                seen_at: Set(None),
                related_id: Set(comment_id),
                ..Default::default()
            }
        }))
        .exec(&db)
        .await
        {
            Ok(r) => r.last_insert_id,
            Err(e) => {
                println!("{} {}", LOG_KEY, e);
                return Err(e);
            }
        };

    println!("last_inserted_notification");
    println!("{}", last_inserted_notification);

    let subscriptions = match subscriptions::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "
                SELECT *
                FROM subscriptions
                WHERE user_id = ANY($1::int[]);
            ",
            [sea_orm::Value::from(
                user_iter.map(|user| user.id).collect::<Vec<i32>>(),
            )],
        ))
        .all(&db)
        .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("{} {}", LOG_KEY, e);
            return Err(e);
        }
    };

    println!("subscriptions");
    println!("{:?}", subscriptions);

    let subscriptions_info = subscriptions
        .into_iter()
        .map(|subscription| {
            web_push::SubscriptionInfo::new(
                subscription.endpoint,
                subscription.p256,
                subscription.auth,
            )
        })
        .collect::<Vec<web_push::SubscriptionInfo>>();

    println!("subscriptions_info");
    println!("{:?}", subscriptions_info);

    for subscription_info in subscriptions_info {
        let mut message = web_push::WebPushMessageBuilder::new(&subscription_info);

        message.set_payload(
            web_push::ContentEncoding::Aes128Gcm,
            "my content".as_bytes(),
        );

        let vapid_sig = web_push::VapidSignatureBuilder::from_base64(
            &std::env::var("SUBSCRIPTION_PRIVATE_KEY").expect("NO_COOKIE_KEY_IN_ENV"),
            web_push::URL_SAFE_NO_PAD,
            &subscription_info,
        )
        .unwrap();

        message.set_vapid_signature(vapid_sig.build().unwrap());

        match isahc_client.send(message.build().unwrap()).await {
            Ok(response) => {
                println!("Notification sent successfully: {:?}", response);
            }
            Err(e) => {
                println!("Error sending notification: {:?}", e);
            }
        }
    }

    return Ok(last_inserted_notification);
}
