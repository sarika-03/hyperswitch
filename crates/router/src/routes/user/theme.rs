use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpRequest, HttpResponse};
use api_models::user::theme as theme_api;
use common_utils::types::user::ThemeLineage;
use masking::Secret;
use router_env::Flow;

use crate::{
    core::{api_locking, user::theme as theme_core},
    routes::AppState,
    services::{api, authentication as auth, authorization::permissions::Permission},
};

pub async fn get_theme_using_lineage(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<ThemeLineage>,
) -> HttpResponse {
    let flow = Flow::GetThemeUsingLineage;
    let lineage = query.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        lineage,
        |state, _, lineage, _| theme_core::get_theme_using_lineage(state, lineage),
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn get_theme_using_theme_id(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::GetThemeUsingThemeId;
    let payload = path.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, _, payload, _| theme_core::get_theme_using_theme_id(state, payload),
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn upload_file_to_theme_storage(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    MultipartForm(payload): MultipartForm<theme_api::UploadFileAssetData>,
) -> HttpResponse {
    let flow = Flow::UploadFileToThemeStorage;
    let theme_id = path.into_inner();
    let payload = theme_api::UploadFileRequest {
        asset_name: payload.asset_name.into_inner(),
        asset_data: Secret::new(payload.asset_data.data.to_vec()),
    };

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, _, payload, _| {
            theme_core::upload_file_to_theme_storage(state, theme_id.clone(), payload)
        },
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn create_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<theme_api::CreateThemeRequest>,
) -> HttpResponse {
    let flow = Flow::CreateTheme;
    let payload = payload.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, _, payload, _| theme_core::create_theme(state, payload),
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn update_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: web::Json<theme_api::UpdateThemeRequest>,
) -> HttpResponse {
    let flow = Flow::UpdateTheme;
    let theme_id = path.into_inner();
    let payload = payload.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, _, payload, _| theme_core::update_theme(state, theme_id.clone(), payload),
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn delete_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::DeleteTheme;
    let theme_id = path.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        theme_id,
        |state, _, theme_id, _| theme_core::delete_theme(state, theme_id),
        &auth::AdminApiAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
pub async fn create_user_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<theme_api::CreateUserThemeRequest>,
) -> HttpResponse {
    let flow = Flow::CreateUserTheme;
    let payload = payload.into_inner();
    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, user: auth::UserFromToken, payload, _| {
            theme_core::create_user_theme(state, user, payload)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeWrite,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
pub async fn get_user_theme_using_theme_id(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::GetUserThemeUsingThemeId;
    let payload = path.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, user: auth::UserFromToken, payload, _| {
            theme_core::get_user_theme_using_theme_id(state, user, payload)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeRead,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn update_user_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    payload: web::Json<theme_api::UpdateThemeRequest>,
) -> HttpResponse {
    let flow = Flow::UpdateUserTheme;
    let theme_id = path.into_inner();
    let payload = payload.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, user: auth::UserFromToken, payload, _| {
            theme_core::update_user_theme(state, theme_id.clone(), user, payload)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeWrite,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn delete_user_theme(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::DeleteUserTheme;
    let theme_id = path.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        theme_id,
        |state, user: auth::UserFromToken, theme_id, _| {
            theme_core::delete_user_theme(state, user, theme_id)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeWrite,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn upload_file_to_user_theme_storage(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    MultipartForm(payload): MultipartForm<theme_api::UploadFileAssetData>,
) -> HttpResponse {
    let flow = Flow::UploadFileToUserThemeStorage;
    let theme_id = path.into_inner();
    let payload = theme_api::UploadFileRequest {
        asset_name: payload.asset_name.into_inner(),
        asset_data: Secret::new(payload.asset_data.data.to_vec()),
    };

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        payload,
        |state, user: auth::UserFromToken, payload, _| {
            theme_core::upload_file_to_user_theme_storage(state, theme_id.clone(), user, payload)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeWrite,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn list_all_themes_in_lineage(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<theme_api::EntityTypeQueryParam>,
) -> HttpResponse {
    let flow = Flow::ListAllThemesInLineage;
    let entity_type = query.into_inner().entity_type;

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        (),
        |state, user: auth::UserFromToken, _payload, _| {
            theme_core::list_all_themes_in_lineage(state, user, entity_type)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeRead,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn get_user_theme_using_lineage(
    state: web::Data<AppState>,
    req: HttpRequest,
    query: web::Query<theme_api::EntityTypeQueryParam>,
) -> HttpResponse {
    let flow = Flow::GetUserThemeUsingLineage;
    let entity_type = query.into_inner().entity_type;

    Box::pin(api::server_wrap(
        flow,
        state,
        &req,
        (),
        |state, user: auth::UserFromToken, _payload, _| {
            theme_core::get_user_theme_using_lineage(state, user, entity_type)
        },
        &auth::JWTAuth {
            permission: Permission::OrganizationThemeRead,
        },
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
