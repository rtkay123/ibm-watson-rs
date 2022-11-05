pub mod errors;

use self::errors::{GetModelError, ListModelsError};

use super::SpeechToText;

use reqwest::{Method, Request, StatusCode, Url, Version};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum ModelID {
    #[deprecated]
    ArArBroadbandModel,
    ArMsBroadbandModel,
    ArMsTelephony,
    CsCzTelephony,
    DeDeBroadbandModel,
    DeDeMultimedia,
    DeDeNarrowbandModel,
    DeDeTelephony,
    EnAuBroadbandModel,
    EnAuMultimedia,
    EnAuNarrowbandModel,
    EnAuTelephony,
    EnGbBroadbandModel,
    EnGbMultimedia,
    EnGbNarrowbandModel,
    EnGbTelephony,
    EnInTelephony,
    EnUsBroadbandModel,
    EnUsMultimedia,
    EnUsNarrowbandModel,
    EnUsShortFormNarrowbandModel,
    EnUsTelephony,
    EnWwMedicalTelephony,
    EsArBroadbandModel,
    EsArNarrowbandModel,
    EsClBroadbandModel,
    EsClNarrowbandModel,
    EsCoBroadbandModel,
    EsCoNarrowbandModel,
    EsEsBroadbandModel,
    EsEsNarrowbandModel,
    EsEsMultimedia,
    EsEsTelephony,
    EsLaTelephony,
    EsMxBroadbandModel,
    EsMxNarrowbandModel,
    EsPeBroadbandModel,
    EsPeNarrowbandModel,
    FrCaBroadbandModel,
    FrCaMultimedia,
    FrCaNarrowbandModel,
    FrCaTelephony,
    FrFrBroadbandModel,
    FrFrMultimedia,
    FrFrNarrowbandModel,
    FrFrTelephony,
    HiInTelephony,
    ItItBroadbandModel,
    ItItNarrowbandModel,
    ItItMultimedia,
    ItItTelephony,
    JaJpBroadbandModel,
    JaJpMultimedia,
    JaJpNarrowbandModel,
    KoKrBroadbandModel,
    KoKrMultimedia,
    KoKrNarrowbandModel,
    KoKrTelephony,
    NlBeTelephony,
    NlNlBroadbandModel,
    NlNlNarrowbandModel,
    NlNlTelephony,
    PtBrBroadbandModel,
    PtBrMultimedia,
    PtBrNarrowbandModel,
    PtBrTelephony,
    SvSeTelephony,
    ZhCnBroadbandModel,
    ZhCnNarrowbandModel,
    ZhCnTelephony,
}

impl ToString for ModelID {
    fn to_string(&self) -> String {
        match self {
            #[allow(deprecated)]
            ModelID::ArArBroadbandModel => "ar-AR_BroadbandModel",
            ModelID::ArMsBroadbandModel => "ar-MS_BroadbandModel",
            ModelID::ArMsTelephony => "ar-MS_Telephony",
            ModelID::CsCzTelephony => "cs-CZ_Telephony",
            ModelID::DeDeBroadbandModel => "cs-CZ_Telephony",
            ModelID::DeDeMultimedia => "de-DE_Multimedia",
            ModelID::DeDeNarrowbandModel => "de-DE_NarrowbandModel",
            ModelID::DeDeTelephony => "de-DE_Telephony",
            ModelID::EnAuBroadbandModel => "en-AU_BroadbandModel",
            ModelID::EnAuMultimedia => "en-AU_Multimedia",
            ModelID::EnAuNarrowbandModel => "en-AU_NarrowbandModel",
            ModelID::EnAuTelephony => "en-AU_Telephony",
            ModelID::EnGbBroadbandModel => "en-GB_BroadbandModel",
            ModelID::EnGbMultimedia => "en-GB_Multimedia",
            ModelID::EnGbNarrowbandModel => "en-GB_NarrowbandModel",
            ModelID::EnGbTelephony => "en-GB_Telephony",
            ModelID::EnInTelephony => "en-IN_Telephony",
            ModelID::EnUsBroadbandModel => "en-US_BroadbandModel",
            ModelID::EnUsMultimedia => "en-US_Multimedia",
            ModelID::EnUsNarrowbandModel => "en-US_NarrowbandModel",
            ModelID::EnUsShortFormNarrowbandModel => "en-US_ShortForm_NarrowbandModel",
            ModelID::EnUsTelephony => "en-US_Telephony",
            ModelID::EnWwMedicalTelephony => "en-WW_Medical_Telephony",
            ModelID::EsArBroadbandModel => "es-AR_BroadbandModel",
            ModelID::EsArNarrowbandModel => "es-AR_NarrowbandModel",
            ModelID::EsClBroadbandModel => "es-CL_BroadbandModel",
            ModelID::EsClNarrowbandModel => "es-CL_NarrowbandModel",
            ModelID::EsCoBroadbandModel => "es-CO_BroadbandModel",
            ModelID::EsCoNarrowbandModel => "es-CO_NarrowbandModel",
            ModelID::EsEsBroadbandModel => "es-ES_BroadbandModel",
            ModelID::EsEsNarrowbandModel => "es-ES_NarrowbandModel",
            ModelID::EsEsMultimedia => "es-ES_Multimedia",
            ModelID::EsEsTelephony => "es-ES_Telephony",
            ModelID::EsLaTelephony => "es-LA_Telephony",
            ModelID::EsMxBroadbandModel => "es-MX_BroadbandModel",
            ModelID::EsMxNarrowbandModel => "es-MX_NarrowbandModel",
            ModelID::EsPeBroadbandModel => "es-PE_BroadbandModel",
            ModelID::EsPeNarrowbandModel => "es-PE_NarrowbandModel",
            ModelID::FrCaBroadbandModel => "fr-CA_BroadbandModel",
            ModelID::FrCaMultimedia => "fr-CA_Multimedia",
            ModelID::FrCaNarrowbandModel => "fr-CA_NarrowbandModel",
            ModelID::FrCaTelephony => "fr-CA_NarrowbandModel",
            ModelID::FrFrBroadbandModel => "fr-FR_BroadbandModel",
            ModelID::FrFrMultimedia => "fr-FR_Multimedia",
            ModelID::FrFrNarrowbandModel => "fr-FR_NarrowbandModel",
            ModelID::FrFrTelephony => "fr-FR_Telephony",
            ModelID::HiInTelephony => "hi-IN_Telephony",
            ModelID::ItItBroadbandModel => "it-IT_BroadbandModel",
            ModelID::ItItNarrowbandModel => "it-IT_NarrowbandModel",
            ModelID::ItItMultimedia => "it-IT_Multimedia",
            ModelID::ItItTelephony => "it-IT_Telephony",
            ModelID::JaJpBroadbandModel => "ja-JP_BroadbandModel",
            ModelID::JaJpMultimedia => "ja-JP_Multimedia",
            ModelID::JaJpNarrowbandModel => "ja-JP_NarrowbandModel",
            ModelID::KoKrBroadbandModel => "ko-KR_BroadbandModel",
            ModelID::KoKrMultimedia => "ko-KR_Multimedia",
            ModelID::KoKrNarrowbandModel => "ko-KR_NarrowbandModel",
            ModelID::KoKrTelephony => "ko-KR_Telephony",
            ModelID::NlBeTelephony => "nl-BE_Telephony",
            ModelID::NlNlBroadbandModel => "nl-NL_BroadbandModel",
            ModelID::NlNlNarrowbandModel => "nl-NL_NarrowbandModel",
            ModelID::NlNlTelephony => "nl-NL_Telephony",
            ModelID::PtBrBroadbandModel => "pt-BR_BroadbandModel",
            ModelID::PtBrMultimedia => "pt-BR_Multimedia",
            ModelID::PtBrNarrowbandModel => "pt-BR_NarrowbandModel",
            ModelID::PtBrTelephony => "pt-BR_Telephony",
            ModelID::SvSeTelephony => "sv-SE_Telephony",
            ModelID::ZhCnBroadbandModel => "zh-CN_BroadbandModel",
            ModelID::ZhCnNarrowbandModel => "zh-CN_NarrowbandModel",
            ModelID::ZhCnTelephony => "zh-CN_Telephony",
        }
        .to_string()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Model {
    pub name: String,
    pub language: String,
    pub url: String,
    pub rate: i64,
    #[serde(rename = "supported_features")]
    pub supported_features: SupportedFeatures,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SupportedFeatures {
    #[serde(rename = "custom_language_model")]
    pub custom_language_model: bool,
    #[serde(rename = "custom_acoustic_model")]
    pub custom_acoustic_model: bool,
    #[serde(rename = "speaker_labels")]
    pub speaker_labels: bool,
}

impl SpeechToText<'_> {
    pub async fn list_models(&self) -> Result<Vec<Model>, ListModelsError> {
        let mut url = Url::parse(self.service_url).unwrap();

        Self::set_models_path(&mut url);

        let mut req = Request::new(Method::GET, url);

        if cfg!(feature = "http2") {
            *req.version_mut() = Version::HTTP_2;
        }

        let client = self.get_client();
        let response = client.execute(req).await?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    models: Vec<Model>,
                }
                let root: Root = response.json().await.unwrap();

                Ok(root.models)
            }
            StatusCode::NOT_ACCEPTABLE => Err(ListModelsError::NotAcceptable406),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(ListModelsError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListModelsError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListModelsError::ServiceUnavailable503),
            _ => Err(ListModelsError::UnmappedResponse(response.status().into())),
        }
    }

    pub async fn get_model(&self, model_id: &ModelID) -> Result<Model, GetModelError> {
        let mut url = Url::parse(self.service_url).unwrap();
        url.set_path(&format!("v1/models/{}", model_id.to_string()));
        let mut req = Request::new(Method::GET, url);

        if cfg!(feature = "http2") {
            *req.version_mut() = Version::HTTP_2;
        }

        let client = self.get_client();
        let response = client.execute(req).await?;
        match response.status() {
            StatusCode::OK => {
                let root: Model = response.json().await.unwrap();
                Ok(root)
            }
            StatusCode::NOT_FOUND => Err(GetModelError::NotFound404(model_id.to_string())),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetModelError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetModelError::ServiceUnavailable503),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(GetModelError::UnsupportedMediaType415),
            StatusCode::NOT_ACCEPTABLE => Err(GetModelError::NotAcceptable406),
            _ => Err(GetModelError::UnmappedResponse(response.status().as_u16())),
        }
    }

    fn set_models_path(uri: &mut Url) {
        uri.set_path("v1/models");
    }
}
