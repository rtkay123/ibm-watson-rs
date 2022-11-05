pub mod errors;

use self::errors::{GetModelError, ListModelsError};

use super::SpeechToText;

use reqwest::{Method, Request, StatusCode, Url, Version};
use serde::Deserialize;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum ModelID {
    #[deprecated]
    ArArBroadband,
    ArMsBroadband,
    ArMsTelephony,
    CsCzTelephony,
    DeDeBroadband,
    DeDeMultimedia,
    DeDeNarrowband,
    DeDeTelephony,
    EnAuBroadband,
    EnAuMultimedia,
    EnAuNarrowband,
    EnAuTelephony,
    EnGbBroadband,
    EnGbMultimedia,
    EnGbNarrowband,
    EnGbTelephony,
    EnInTelephony,
    EnUsBroadband,
    EnUsMultimedia,
    EnUsNarrowband,
    EnUsShortFormNarrowband,
    EnUsTelephony,
    EnWwMedicalTelephony,
    EsArBroadband,
    EsArNarrowband,
    EsClBroadband,
    EsClNarrowband,
    EsCoBroadband,
    EsCoNarrowband,
    EsEsBroadband,
    EsEsNarrowband,
    EsEsMultimedia,
    EsEsTelephony,
    EsLaTelephony,
    EsMxBroadband,
    EsMxNarrowband,
    EsPeBroadband,
    EsPeNarrowband,
    FrCaBroadband,
    FrCaMultimedia,
    FrCaNarrowband,
    FrCaTelephony,
    FrFrBroadband,
    FrFrMultimedia,
    FrFrNarrowband,
    FrFrTelephony,
    HiInTelephony,
    ItItBroadband,
    ItItNarrowband,
    ItItMultimedia,
    ItItTelephony,
    JaJpBroadband,
    JaJpMultimedia,
    JaJpNarrowband,
    KoKrBroadband,
    KoKrMultimedia,
    KoKrNarrowband,
    KoKrTelephony,
    NlBeTelephony,
    NlNlBroadband,
    NlNlNarrowband,
    NlNlTelephony,
    PtBrBroadband,
    PtBrMultimedia,
    PtBrNarrowband,
    PtBrTelephony,
    SvSeTelephony,
    ZhCnBroadband,
    ZhCnNarrowband,
    ZhCnTelephony,
}

impl ToString for ModelID {
    fn to_string(&self) -> String {
        match self {
            #[allow(deprecated)]
            ModelID::ArArBroadband => "ar-AR_BroadbandModel",
            ModelID::ArMsBroadband => "ar-MS_BroadbandModel",
            ModelID::ArMsTelephony => "ar-MS_Telephony",
            ModelID::CsCzTelephony => "cs-CZ_Telephony",
            ModelID::DeDeBroadband => "cs-CZ_Telephony",
            ModelID::DeDeMultimedia => "de-DE_Multimedia",
            ModelID::DeDeNarrowband => "de-DE_NarrowbandModel",
            ModelID::DeDeTelephony => "de-DE_Telephony",
            ModelID::EnAuBroadband => "en-AU_BroadbandModel",
            ModelID::EnAuMultimedia => "en-AU_Multimedia",
            ModelID::EnAuNarrowband => "en-AU_NarrowbandModel",
            ModelID::EnAuTelephony => "en-AU_Telephony",
            ModelID::EnGbBroadband => "en-GB_BroadbandModel",
            ModelID::EnGbMultimedia => "en-GB_Multimedia",
            ModelID::EnGbNarrowband => "en-GB_NarrowbandModel",
            ModelID::EnGbTelephony => "en-GB_Telephony",
            ModelID::EnInTelephony => "en-IN_Telephony",
            ModelID::EnUsBroadband => "en-US_BroadbandModel",
            ModelID::EnUsMultimedia => "en-US_Multimedia",
            ModelID::EnUsNarrowband => "en-US_NarrowbandModel",
            ModelID::EnUsShortFormNarrowband => "en-US_ShortForm_NarrowbandModel",
            ModelID::EnUsTelephony => "en-US_Telephony",
            ModelID::EnWwMedicalTelephony => "en-WW_Medical_Telephony",
            ModelID::EsArBroadband => "es-AR_BroadbandModel",
            ModelID::EsArNarrowband => "es-AR_NarrowbandModel",
            ModelID::EsClBroadband => "es-CL_BroadbandModel",
            ModelID::EsClNarrowband => "es-CL_NarrowbandModel",
            ModelID::EsCoBroadband => "es-CO_BroadbandModel",
            ModelID::EsCoNarrowband => "es-CO_NarrowbandModel",
            ModelID::EsEsBroadband => "es-ES_BroadbandModel",
            ModelID::EsEsNarrowband => "es-ES_NarrowbandModel",
            ModelID::EsEsMultimedia => "es-ES_Multimedia",
            ModelID::EsEsTelephony => "es-ES_Telephony",
            ModelID::EsLaTelephony => "es-LA_Telephony",
            ModelID::EsMxBroadband => "es-MX_BroadbandModel",
            ModelID::EsMxNarrowband => "es-MX_NarrowbandModel",
            ModelID::EsPeBroadband => "es-PE_BroadbandModel",
            ModelID::EsPeNarrowband => "es-PE_NarrowbandModel",
            ModelID::FrCaBroadband => "fr-CA_BroadbandModel",
            ModelID::FrCaMultimedia => "fr-CA_Multimedia",
            ModelID::FrCaNarrowband => "fr-CA_NarrowbandModel",
            ModelID::FrCaTelephony => "fr-CA_NarrowbandModel",
            ModelID::FrFrBroadband => "fr-FR_BroadbandModel",
            ModelID::FrFrMultimedia => "fr-FR_Multimedia",
            ModelID::FrFrNarrowband => "fr-FR_NarrowbandModel",
            ModelID::FrFrTelephony => "fr-FR_Telephony",
            ModelID::HiInTelephony => "hi-IN_Telephony",
            ModelID::ItItBroadband => "it-IT_BroadbandModel",
            ModelID::ItItNarrowband => "it-IT_NarrowbandModel",
            ModelID::ItItMultimedia => "it-IT_Multimedia",
            ModelID::ItItTelephony => "it-IT_Telephony",
            ModelID::JaJpBroadband => "ja-JP_BroadbandModel",
            ModelID::JaJpMultimedia => "ja-JP_Multimedia",
            ModelID::JaJpNarrowband => "ja-JP_NarrowbandModel",
            ModelID::KoKrBroadband => "ko-KR_BroadbandModel",
            ModelID::KoKrMultimedia => "ko-KR_Multimedia",
            ModelID::KoKrNarrowband => "ko-KR_NarrowbandModel",
            ModelID::KoKrTelephony => "ko-KR_Telephony",
            ModelID::NlBeTelephony => "nl-BE_Telephony",
            ModelID::NlNlBroadband => "nl-NL_BroadbandModel",
            ModelID::NlNlNarrowband => "nl-NL_NarrowbandModel",
            ModelID::NlNlTelephony => "nl-NL_Telephony",
            ModelID::PtBrBroadband => "pt-BR_BroadbandModel",
            ModelID::PtBrMultimedia => "pt-BR_Multimedia",
            ModelID::PtBrNarrowband => "pt-BR_NarrowbandModel",
            ModelID::PtBrTelephony => "pt-BR_Telephony",
            ModelID::SvSeTelephony => "sv-SE_Telephony",
            ModelID::ZhCnBroadband => "zh-CN_BroadbandModel",
            ModelID::ZhCnNarrowband => "zh-CN_NarrowbandModel",
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
