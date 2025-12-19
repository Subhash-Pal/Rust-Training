/*5. Futures and Async Syntax
🔹 Concept

async functions return Futures

Futures are executed by an async runtime (like Tokio)

✅ Example: Async function with Tokio
*/


/*use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() {
    task_one().await;
    task_two().await;
}

async fn task_one() {
    sleep(Duration::from_secs(5)).await;
    println!("Task one done");
}

async fn task_two() {
    sleep(Duration::from_secs(1)).await;
    println!("Task two done");
}
*/

/*
📌 Key points

async fn → returns a Future

.await pauses without blocking threads

Requires an async runtime (tokio)
*/



/*
How to make it truly concurrent 🚀
✅ Correct concurrent version
*/
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task one done");
    });

    let t2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task two done");
    });

    t1.await.unwrap();
    t2.await.unwrap();
}

/*
⏱️ Timeline now

Both tasks start at the same time

Both sleep for 1 second

Total time ≈ 1 second

⚠️ Output order may vary:

Task two done
Task one done
*/