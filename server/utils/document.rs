pub struct Document {}

pub struct DocumentProps {
    pub description: String,
    pub title: String,
    pub wasm_assets: String,
    pub content: String,
}

impl Document {
    pub fn new(document_props: DocumentProps) -> String {
        format!(
            r#"
			<!DOCTYPE html>
			<html lang="en">
				<head>
					<meta charset="UTF-8" />
					<meta name="description" content="{}">
					<meta name="theme-color" content='#0d6efd' />
					<meta http-equiv="X-UA-Compatible" content="IE=edge" />
					<meta name="viewport" content="width=device-width, initial-scale=1.0" />
					<link rel="stylesheet" href="/assets/bootstrap.css" />
					<script defer src="/assets/bootstrap.js" type="text/javascript"></script>
					<script defer src="/assets/{}"></script>
					<title>{} | mockbuster</title>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            document_props.description,
            document_props.wasm_assets,
            document_props.title,
            document_props.content
        )
    }
}
