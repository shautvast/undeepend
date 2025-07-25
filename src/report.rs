use crate::maven::project::Project;
use maud::{Markup, PreEscaped, html};

impl Project {
    pub fn generate_dependency_html(&self) -> String {
        let html = html! {
            (PreEscaped(r#"
                <head>
                  <meta charset="UTF-8">
                  <meta name="viewport" content="width=device-width, initial-scale=1.0">
                  <title>Project Dependencies</title>
                  <style>
                        body {
                            font-family: sans-serif;
                        }
                        table {
                        width: 100%;
                        border-collapse: collapse;
                    }
                    th, td {
                        border: 1px solid #ddd;
                        padding: 8px;
                        text-align: left;
                    }
                    th {
                        background-color: #f2f2f2;
                    }
                  </style>
                </head>
"#))
        h1{"Project Dependencies"}
        table{
            thead{
                tr {
                    th{"Group ID"}
                    th{"Artifact ID"}
                    th{"Version"}
                }
            }
            tbody{
                @for dependency in &self.get_dependencies(&self.root) {
                    tr {
                            td { (dependency.group_id) }
                            td { (dependency.artifact_id) }
                            td { (dependency.version.clone().unwrap()) }
                        }
                }
            }
        }
        };
        html.into_string()
    }
}
