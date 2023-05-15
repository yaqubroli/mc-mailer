use chrono::Utc;
use crate::secrets::*;
use crate::verification::*;
use crate::email::*;
use actix_web::{web, Responder, HttpResponse};

pub async fn index() -> impl Responder {
    // generate a nonce verification request, and send it as an email
    let request_proto = VerificationRequestProto {
        email: String::from("jpw24"),
        minecraft_username: String::from("jacobroly")
    };
    let request = request_proto.hydrate();
    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            HTML_ON_EMAIL_SENT_1.to_string() + &request.minecraft_username + HTML_ON_EMAIL_SENT_2 + &request.email + HTML_ON_EMAIL_SENT_3
        );
    // actix_rt::spawn(send_verification_email(request));
    println!("{}", request.as_code());
    return response;
}

pub async fn send_sta(form: web::Form<VerificationRequestProto>) -> impl Responder {
    let request = form.into_inner().hydrate();
    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            HTML_ON_EMAIL_SENT_1.to_string() + &request.minecraft_username + HTML_ON_EMAIL_SENT_2 + &request.email + HTML_ON_EMAIL_SENT_3
        );
    actix_rt::spawn(send_verification_email(request));
    return response;
}

pub async fn send_written(form: web::Form<WrittenReasonRequest>) -> impl Responder {
    let response = HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HTML_ON_WRITTEN_REQUEST_SENT.to_string());
    actix_rt::spawn(send_written_reason_email(form.into_inner()));
    return response;
}

pub async fn verify(path: web::Path<String>) -> impl Responder {
    let code = path.into_inner();
    let receipt = VerificationReceipt::from_code(code);
    if (receipt.valid) {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(HTML_ON_EMAIL_VERIFIED_1.to_string() + &receipt.minecraft_username + HTML_ON_EMAIL_VERIFIED_2);
    } else {
        return HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(HTML_ON_EMAIL_VERIFICATION_FAILED.to_string());
    }
}

