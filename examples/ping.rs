use cloudflare_slash::*;
use worker::*;
use cloudflare_slash::command::CommandOption;

#[derive(CommandOption)]
#[command_enum]
enum ExampleCommandEnum {
    #[description = "First option"]
    OptionA,
    OptionB,
    OptionC
}

#[derive(CommandOption)]
enum ExampleCommand {
    #[description = "Return Pong"]
    Ping{
        echo: String
    },
    Echo{
        #[description = "value to echo"]
        echo: String,
        #[description = "Options are not required"]
        optional: Option<f64>,
        #[command_enum]
        #[description = "Tag an argument with command_enum to convert it to an option list"]
        checklist: ExampleCommandEnum,
        #[description = "CommandOptions are converted to Command Groups"]
        subcommand: ExampleSubCommand
    }
}

#[derive(CommandOption)]
enum ExampleSubCommand{
    #[description = "Foo command"]
    Foo,
    #[description = "Bar command"]
    Bar
}


impl command::CommandHandler for ExampleCommand {
    fn handle(args: Self, req: interactions::InteractionRequest) -> interactions::InteractionResponse{
        interactions::InteractionResponse::Pong
    }
}

#[event(fetch)]
pub async fn handle_req(req: Request, env: Env) -> Result<Response> {
    cloudflare_slash::handle_request::<ExampleCommand>(req, env).await
}

#[cfg(not(target_arch="wasm32"))]
pub fn main() {
    if let Ok(json) = serde_json::to_string_pretty(&ExampleCommand::to_value()){
        println!("Command JSON: {}", json);
    }
}