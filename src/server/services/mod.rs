use diesel::MysqlConnection;
use diesel::result::Error;
use leptos::ServerFnError;
use leptos::use_context;

use crate::beans::server_error_type::CustomError;

pub mod novel_service;
pub mod chapter_service;

pub fn get_db_connexion_wrapper<F, Out>(func: F) -> Result<Out, ServerFnError<CustomError>>
    where F: FnOnce(&mut MysqlConnection) -> Result<Out, Error>
{
    let server_context = use_context::<crate::server::db::DbPool>();
    ServerFnError::WrappedServerError(Error::NotFound);
    match server_context {
        Some(pool) => {
            let mut conn = pool.get().map_err(|e| ServerFnError::<CustomError>::ServerError(e.to_string()))?;
            Ok(
                func(&mut *conn)
                    .map_err(|e| match e {
                        Error::NotFound => ServerFnError::<CustomError>::WrappedServerError(CustomError::NotFound),
                        _ => ServerFnError::<CustomError>::ServerError(e.to_string())
                    })?
            )
        }
        None => Err(ServerFnError::ServerError("Context error".to_string()))
    }
}
