use super::m20231017_010052_ratings::Ratings;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Movies::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Movies::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Movies::BackdropPath).string())
                    .col(ColumnDef::new(Movies::Title).string().not_null())
                    .col(ColumnDef::new(Movies::Overview).text().not_null())
                    .col(ColumnDef::new(Movies::PosterPath).string().not_null())
                    .col(ColumnDef::new(Movies::ReleaseDate).date())
                    .col(
                        ColumnDef::new(Movies::TmdbId)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Movies::TmdbVoteAverage).decimal().not_null())
                    .col(ColumnDef::new(Movies::TmdbVoteCount).integer().not_null())
                    .col(
                        ColumnDef::new(Movies::PostgresVoteAverage)
                            .decimal()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Movies::PostgresVoteCount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Movies::CreatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Movies::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default("now()")
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let conn = manager.get_connection();

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("movies-created_at-index")
                    .table(Movies::Table)
                    .col(Movies::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("ratings-movies-tmdb-id")
                    .from(Ratings::Table, Ratings::TmdbId)
                    .to(Movies::Table, Movies::TmdbId)
                    .to_owned(),
            )
            .await?;

        #[cfg(any(debug_assertions, test))]
        {
            conn.execute_unprepared(
				"
				INSERT INTO movies (
					backdrop_path, 
					overview,
					poster_path,
					postgres_vote_average,
					postgres_vote_count,
					release_date,
					tmdb_id,
					tmdb_vote_average,
					tmdb_vote_count,
					title
				) VALUES
				(
					'/hZkgoQYus5vegHoetLkCJzb17zJ.jpg', 
					'A ticking-time-bomb insomniac and a slippery soap salesman channel primal male aggression into a shocking new form of therapy. Their concept catches on, with underground \"fight clubs\" forming in every town, until an eccentric gets in the way and ignites an out-of-control spiral toward oblivion.', 
					'/pB8BM7pdSp6B6Ih7QZ4DrQ3PmJK.jpg', 
					10.0,
					3,
					'1999-10-15', 
					550, 
					8.5, 
					27707, 
					'Fight Club'
				),
				(
					'/4bspHpwUVbMBYu5afukIo7n4oN.jpg', 
					'Taking a wrong turn, travelers find themselves trapped in a mysterious house. One horror after another threatens them as the sorcerer who lives within needs sacrifices to give eternal life to his beautiful bride.', 
					'/mmJN1NvMIVQaWvO3iWvdSppfFy3.jpg', 
					10.0,
					3,
					'1986-05-14', 
					26679, 
					5.0, 
					79, 
					'Spookies'
				),
				(
					'/1TUg5pO1VZ4B0Q1amk3OlXvlpXV.jpg', 
					'Simba idolizes his father, King Mufasa, and takes to heart his own royal destiny. But not everyone in the kingdom celebrates the new cub''s arrival. Scar, Mufasa''s brother—and former heir to the throne—has plans of his own. The battle for Pride Rock is ravaged with betrayal, tragedy and drama, ultimately resulting in Simba''s exile. With help from a curious pair of newfound friends, Simba will have to figure out how to grow up and take back what is rightfully his.', 
					'/dzBtMocZuJbjLOXvrl4zGYigDzh.jpg', 
					5.0,
					3,
					'2019-07-12', 
					420818, 
					7.0, 
					9460, 
					'The Lion King'
				),
				(
					'/y0sXAOehV5zx1E64HBSyjDSZK22.jpg', 
					'Edmond Dantés''s life and plans to marry the beautiful Mercedes are shattered when his best friend, Fernand, deceives him. After spending 13 miserable years in prison, Dantés escapes with the help of a fellow inmate and plots his revenge, cleverly insinuating himself into the French nobility.', 
					'/ifMgDAUXVQLY4DeOu3VTTi55jSP.jpg', 
					7.0,
					3,
					'2002-01-23', 
					11362, 
					7.5, 
					1626, 
					'The Count of Monte Cristo'
				);
				"
			)
			.await?;

            conn.execute_unprepared(
                "
						INSERT INTO ratings (user_id, score, tmdb_id) VALUES
						(1, 10, 550),
						(2, 10, 550),
						(3, 10, 550),
						(1, 10, 26679),
						(2, 10, 26679),
						(3, 10, 26679),
						(1, 9.0, 11362),
						(2, 7.5, 11362),
						(3, 5.0, 11362),
						(1, 7.5, 420818),
						(2, 2.0, 420818),
						(3, 6.0, 420818);
					",
            )
            .await?;
        }

        conn.execute_unprepared(
            "
				CREATE TRIGGER trigger_update_movies_updated_at
				BEFORE UPDATE
				ON movies
				FOR EACH ROW
				EXECUTE FUNCTION update_updated_at_column();
			",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ratings::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Movies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Movies {
    Table,
    Id,
    BackdropPath,
    Title,
    Overview,
    ReleaseDate,
    TmdbId,
    TmdbVoteAverage,
    TmdbVoteCount,
    PostgresVoteAverage,
    PostgresVoteCount,
    PosterPath,
    CreatedAt,
    UpdatedAt,
}
