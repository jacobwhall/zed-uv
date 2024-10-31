use serde::{Deserialize, Serialize};
use zed_extension_api::{
    self as zed, http_client, serde_json, SlashCommand, SlashCommandArgumentCompletion,
    SlashCommandOutput,
};

#[derive(Serialize)]
struct UvRequest {
    project: String,
    command: String,
    args: Vec<String>,
}

#[derive(Deserialize)]
struct UvResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

struct UvExtension;

impl zed::Extension for UvExtension {
    fn new() -> Self {
        UvExtension
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "uv" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "Add".to_string(),
                    new_text: "add".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Init".to_string(),
                    new_text: "init".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Remove".to_string(),
                    new_text: "remove".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Sync".to_string(),
                    new_text: "sync".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Lock".to_string(),
                    new_text: "lock".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Export".to_string(),
                    new_text: "export".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Tree".to_string(),
                    new_text: "tree".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Tool".to_string(),
                    new_text: "tool".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Python".to_string(),
                    new_text: "python".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Pip".to_string(),
                    new_text: "pip".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Venv".to_string(),
                    new_text: "venv".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Build".to_string(),
                    new_text: "build".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Wheels".to_string(),
                    new_text: "wheels".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Publish".to_string(),
                    new_text: "publish".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Cache".to_string(),
                    new_text: "cache".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Self".to_string(),
                    new_text: "self".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Version".to_string(),
                    new_text: "version".to_string(),
                    run_command: true,
                },
                SlashCommandArgumentCompletion {
                    label: "Help".to_string(),
                    new_text: "help".to_string(),
                    run_command: true,
                },
            ]),
            command => Err(format!("Unknown slash command: \"{command}\"")),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&zed_extension_api::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        let project_path = match worktree {
            Some(wt) => wt.root_path(),
            None => return Err("Could not determine project directory.".to_string()),
        };

        match command.name.as_str() {
            "uv" => {
                let command = args.first().ok_or("No command specified")?;
                let remaining_args = args[1..].to_vec();

                let payload = serde_json::to_vec(&UvRequest {
                    project: project_path,
                    command: command.to_string(),
                    args: remaining_args,
                })
                .map_err(|e| e.to_string())?;

                let request = http_client::HttpRequest::builder()
                    .method(http_client::HttpMethod::Post)
                    .url("http://localhost:9876/uv")
                    .header("Content-Type", "application/json")
                    .body(payload)
                    .build()?;

                let response = request.fetch()?;
                let uv_response: UvResponse =
                    serde_json::from_slice(&response.body).map_err(|e| e.to_string())?;

                Ok(SlashCommandOutput {
                    text: if uv_response.success {
                        uv_response.stdout
                    } else {
                        format!("Error: {}", uv_response.stderr)
                    },
                    sections: vec![],
                })
            }
            _ => Err("Unknown command".into()),
        }
    }
}

zed::register_extension!(UvExtension);
