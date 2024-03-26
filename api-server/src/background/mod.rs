mod movie_tracking;
use self::movie_tracking::movie_tracking;
use crate::models::context::ContextPointer;
use log::error;

pub async fn background(context: ContextPointer) {
    if let Err(error) = movie_tracking(context).await {
        error!("MovieTracking error: {:?}", error);
    }
}
