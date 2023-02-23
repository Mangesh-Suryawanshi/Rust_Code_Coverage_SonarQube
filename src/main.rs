#[macro_use] extern  crate rocket;
use std::{fs::{OpenOptions}, io::{Write, BufRead, BufReader}};
use rocket::{serde::{Deserialize, json::Json}};
//use std::io::BufReader<std::fs::File>;
//use std::io::BufRead;
//use std::{io::{BufReader}};
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to MS todo app using rust"
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new().read(true).append(true).create(true).open("task.txt").expect("task.txt not accessible.");
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("Unable to write to task.txt");
    "Task added successfully."
}

#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new().read(true).append(true).create(true).open("task.txt")
    .expect("unable to access task.txt");
    let reader = BufReader::new(tasks); 
    Json(reader.lines().map(|line|line.expect("Could not read line")).collect())
    //let reader = BufReader::new(tasks);
    //Json(reader.read_line().map(|line|line.expect("Could not read line")).collect())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![index, add_task, read_tasks])
}