mod movie_tracking;

use crate::context::ContextPointer;

use self::movie_tracking::movie_tracking;

pub async fn background(context: ContextPointer) {
    if let Err(error) = movie_tracking(context).await {
        println!("Error: {:?}", error);
    }
}
