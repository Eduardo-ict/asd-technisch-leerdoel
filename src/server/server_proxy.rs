use crate::server::rpc_broker;

// Verwerkt de aanvraag op basis van de functie naam en argumenten
pub fn process_request(request: &str) -> String {
    let parts: Vec<&str> = request.split(':').collect();

    let function_name = parts[0];
    let args = parts[1];

    // Split de argumenten op basis van komma's
    let args: Vec<i32> = split_args(args, parse_i32);

    // Roep de juiste functie aan
    rpc_broker::invoke_function(function_name, &args)
}

fn split_args<T, F>(args: &str, parser: F) -> Vec<T>
where
    F: Fn(&str) -> Option<T>,
{
    args.split(',')
        .filter_map(parser)
        .collect()
}

fn parse_i32(arg: &str) -> Option<i32> {
    arg.parse::<i32>().ok()
}
