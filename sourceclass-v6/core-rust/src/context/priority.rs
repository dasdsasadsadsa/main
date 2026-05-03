pub fn get_priority_for_role(role: &str) -> (String, String) {
    match role {
        "Brain" => (
            String::from("MustSend"),
            String::from("Core logic file - essential for understanding project behavior"),
        ),
        "Entry" => (
            String::from("MustSend"),
            String::from("Entry point - execution starts here"),
        ),
        "Interface" => (
            String::from("ShouldSend"),
            String::from("API/routes interface - important for understanding external interactions"),
        ),
        "Connector" => (
            String::from("ShouldSend"),
            String::from("Integration layer - shows external dependencies"),
        ),
        "Config" => (
            String::from("SendIfNeeded"),
            String::from("Configuration - reference when needed for context"),
        ),
        "Output" => (
            String::from("SendIfNeeded"),
            String::from("Output/rendering - useful for understanding data flow"),
        ),
        "Utility" => (
            String::from("IgnoreFirst"),
            String::from("Helper function - read only if specific functionality is needed"),
        ),
        "Dangerous" => (
            String::from("NeverSend"),
            String::from("Security-sensitive - do not send to LLM without explicit approval"),
        ),
        _ => (
            String::from("IgnoreFirst"),
            String::from("Unclassified - low priority for initial analysis"),
        ),
    }
}
