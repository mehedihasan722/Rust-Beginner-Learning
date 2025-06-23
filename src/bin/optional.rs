struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response: Survey = Survey {
        q1: Some(5),
        q2: Some(true),
        q3: Some("Great service!".to_string()),
    };

    match response.q1 {
        Some(score) => println!("Question 1 score: {}", score),
        None => println!("Question 1 not answered"),
    }
    match response.q2 {
        Some(answer) => println!("Question 2 answer: {}", answer),
        None => println!("Question 2 not answered"),
    }
    match response.q3 {
        Some(comment) => println!("Question 3 comment: {}", comment),
        None => println!("Question 3 not answered"),
    }
}
