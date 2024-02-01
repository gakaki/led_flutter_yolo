use salvo::prelude::*;
use crate::controller::experiments_controller::{add_experiment, get_experiments, show_experiment, update_experiment_start_and_end};
use crate::controller::users_controller::{get_users, add_user};

pub fn get_router() -> Router {
    Router::new()
        .push(
            Router::with_path("users")
                .get(get_users)
                .post(add_user)
        )
        .push(
            Router::with_path("experiments")
                .get(get_experiments)
                .post(add_experiment)
                .push(
                    Router::with_path("<id>")
                        .get(show_experiment)
                )
                .push(
                    Router::with_path("<id>")
                        .patch(update_experiment_start_and_end)
                )
        )
}
