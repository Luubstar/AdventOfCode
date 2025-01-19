let year = variable::get("year");
let day = variable::get("day");
let cookie = variable::get("cookie");

let url = "https://adventofcode.com/" + year.to_string() + "/day/" + day.to_string() + "/input";
let value = system::command("bash", ["-c", "curl --cookie session=" + cookie + " " + url + " > input.txt"]);