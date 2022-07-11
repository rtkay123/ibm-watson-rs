use hyper::{
    body::Buf,
    header::{HeaderValue, AUTHORIZATION},
    Body, Client, Method, Request, StatusCode,
};
use serde::{Deserialize, Serialize};

pub mod errors;
use url::Url;

use crate::tts::voices::errors::GetVoiceError;

use self::errors::ListVoicesError;

use super::{customisations::CustomModel, TextToSpeech};
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Voice {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "gender")]
    pub gender: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "language")]
    pub language: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "customizable")]
    pub customisable: bool,
    #[serde(rename = "supported_features")]
    pub supported_features: Box<SupportedFeatures>,
    #[serde(rename = "customization", skip_serializing_if = "Option::is_none")]
    pub customisation: Option<Box<CustomModel>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct SupportedFeatures {
    #[serde(rename = "custom_pronunciation")]
    pub custom_pronunciation: bool,
    #[serde(rename = "voice_transformation")]
    pub voice_transformation: bool,
}

#[derive(Default)]
#[non_exhaustive]
pub enum WatsonVoice {
    ArMsOmar,
    CsCzAlena,
    DeDeBirgitV3,
    DeDeDieterV3,
    DeDeErikaV3,
    EnAuCraig,
    EnAuMadison,
    EnAuSteve,
    EnGbCharlotteV3,
    EnGbJamesV3,
    EnGbKateV3,
    EnUsAllisonV3,
    EnUsEmilyV3,
    EnUsHenryV3,
    EnUsKevinV3,
    EnUsLisaV3,
    #[default]
    EnUsMichaelV3,
    EnUsOliviaV3,
    EsEsEnriqueV3,
    EsEsLauraV3,
    EsLaSofiaV3,
    EsUsSofiaV3,
    FrCaLouiseV3,
    FrFrNicolasV3,
    FrFrReneeV3,
    ItItFrancescaV3,
    JaJpEmiV3,
    KoKrHyunjun,
    KoKrSiWoo,
    KoKrYoungmi,
    KoKrYuna,
    NlBeAdele,
    NlBeBram,
    NlNlEmma,
    NlNlLiam,
    PtBrIsabelaV3,
    SvSeIngrid,
    ZhCnLiNa,
    ZhCnWangWei,
    ZhCnZhangJing,
}

impl ToString for WatsonVoice {
    fn to_string(&self) -> String {
        match &self {
            WatsonVoice::ArMsOmar => todo!(),
            WatsonVoice::CsCzAlena => todo!(),
            WatsonVoice::DeDeBirgitV3 => todo!(),
            WatsonVoice::DeDeDieterV3 => todo!(),
            WatsonVoice::DeDeErikaV3 => todo!(),
            WatsonVoice::EnAuCraig => todo!(),
            WatsonVoice::EnAuMadison => todo!(),
            WatsonVoice::EnAuSteve => todo!(),
            WatsonVoice::EnGbCharlotteV3 => todo!(),
            WatsonVoice::EnGbJamesV3 => todo!(),
            WatsonVoice::EnGbKateV3 => todo!(),
            WatsonVoice::EnUsAllisonV3 => todo!(),
            WatsonVoice::EnUsEmilyV3 => todo!(),
            WatsonVoice::EnUsHenryV3 => todo!(),
            WatsonVoice::EnUsKevinV3 => todo!(),
            WatsonVoice::EnUsLisaV3 => todo!(),
            WatsonVoice::EnUsMichaelV3 => todo!(),
            WatsonVoice::EnUsOliviaV3 => todo!(),
            WatsonVoice::EsEsEnriqueV3 => todo!(),
            WatsonVoice::EsEsLauraV3 => todo!(),
            WatsonVoice::EsLaSofiaV3 => todo!(),
            WatsonVoice::EsUsSofiaV3 => todo!(),
            WatsonVoice::FrCaLouiseV3 => todo!(),
            WatsonVoice::FrFrNicolasV3 => todo!(),
            WatsonVoice::FrFrReneeV3 => todo!(),
            WatsonVoice::ItItFrancescaV3 => todo!(),
            WatsonVoice::JaJpEmiV3 => todo!(),
            WatsonVoice::KoKrHyunjun => todo!(),
            WatsonVoice::KoKrSiWoo => todo!(),
            WatsonVoice::KoKrYoungmi => todo!(),
            WatsonVoice::KoKrYuna => todo!(),
            WatsonVoice::NlBeAdele => todo!(),
            WatsonVoice::NlBeBram => todo!(),
            WatsonVoice::NlNlEmma => todo!(),
            WatsonVoice::NlNlLiam => todo!(),
            WatsonVoice::PtBrIsabelaV3 => todo!(),
            WatsonVoice::SvSeIngrid => todo!(),
            WatsonVoice::ZhCnLiNa => todo!(),
            WatsonVoice::ZhCnWangWei => todo!(),
            WatsonVoice::ZhCnZhangJing => todo!(),
        }
    }
}

impl WatsonVoice {
    pub fn id(&self) -> &str {
        match &self {
            WatsonVoice::ArMsOmar => "ar-MS_OmarVoice",
            WatsonVoice::CsCzAlena => "cs-CZ_AlenaVoice",
            WatsonVoice::DeDeBirgitV3 => "de-DE_BirgitV3Voice",
            WatsonVoice::DeDeDieterV3 => "de-DE_DieterV3Voice",
            WatsonVoice::DeDeErikaV3 => "de-DE_ErikaV3Voice",
            WatsonVoice::EnAuCraig => "en-AU_CraigVoice",
            WatsonVoice::EnAuMadison => "en-AU_MadisonVoice",
            WatsonVoice::EnAuSteve => "en-AU_SteveVoice",
            WatsonVoice::EnGbCharlotteV3 => "en-GB_CharlotteV3Voice",
            WatsonVoice::EnGbJamesV3 => "en-GB_JamesV3Voice",
            WatsonVoice::EnGbKateV3 => "en-GB_KateV3Voice",
            WatsonVoice::EnUsAllisonV3 => "en-US_AllisonV3Voice",
            WatsonVoice::EnUsEmilyV3 => "en-US_EmilyV3Voice",
            WatsonVoice::EnUsHenryV3 => "en-US_HenryV3Voice",
            WatsonVoice::EnUsKevinV3 => "en-US_KevinV3Voice",
            WatsonVoice::EnUsLisaV3 => "en-US_LisaV3Voice",
            WatsonVoice::EnUsMichaelV3 => "en-US_MichaelV3Voice",
            WatsonVoice::EnUsOliviaV3 => "en-US_OliviaV3Voice",
            WatsonVoice::EsEsEnriqueV3 => "es-ES_EnriqueV3Voice",
            WatsonVoice::EsEsLauraV3 => "es-ES_LauraV3Voice",
            WatsonVoice::EsLaSofiaV3 => "es-LA_SofiaV3Voice",
            WatsonVoice::EsUsSofiaV3 => "es-US_SofiaV3Voice",
            WatsonVoice::FrCaLouiseV3 => "fr-CA_LouiseV3Voice",
            WatsonVoice::FrFrNicolasV3 => "fr-FR_NicolasV3Voice",
            WatsonVoice::FrFrReneeV3 => "fr-FR_ReneeV3Voice",
            WatsonVoice::ItItFrancescaV3 => "it-IT_FrancescaV3Voice",
            WatsonVoice::JaJpEmiV3 => "ja-JP_EmiV3Voice",
            WatsonVoice::KoKrHyunjun => "ko-KR_HyunjunVoice",
            WatsonVoice::KoKrSiWoo => "ko-KR_SiWooVoice",
            WatsonVoice::KoKrYoungmi => "ko-KR_YoungmiVoice",
            WatsonVoice::KoKrYuna => "ko-KR_YunaVoice",
            WatsonVoice::NlBeAdele => "nl-BE_AdeleVoice",
            WatsonVoice::NlBeBram => "nl-BE_BramVoice",
            WatsonVoice::NlNlEmma => "nl-NL_EmmaVoice",
            WatsonVoice::NlNlLiam => "nl-NL_LiamVoice",
            WatsonVoice::PtBrIsabelaV3 => "pt-BR_IsabelaV3Voice",
            WatsonVoice::SvSeIngrid => "sv-SE_IngridVoice",
            WatsonVoice::ZhCnLiNa => "zh-CN_LiNaVoice",
            WatsonVoice::ZhCnWangWei => "zh-CN_WangWeiVoice",
            WatsonVoice::ZhCnZhangJing => "zh-CN_ZhangJingVoice",
        }
    }
}

impl TextToSpeech<'_> {
    pub async fn list_voices(&self) -> Result<Vec<Voice>, ListVoicesError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        let req = Request::builder()
            .uri(url.to_string())
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| ListVoicesError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client
            .request(req)
            .await
            .map_err(|e| ListVoicesError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    voices: Vec<Voice>,
                }
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Root = serde_json::from_reader(body.reader()).unwrap();

                Ok(root.voices)
            }
            StatusCode::NOT_ACCEPTABLE => Err(ListVoicesError::NotAcceptable),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(ListVoicesError::UnsupportedMediaType),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListVoicesError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListVoicesError::ServiceUnavailable),
            _ => {
                unreachable!()
            }
        }
    }

    fn set_voices_path(uri: &mut Url) {
        uri.set_path("v1/voices");
    }

    pub async fn get_voice(
        &self,
        voice: WatsonVoice,
        customisation_id: Option<&str>,
    ) -> Result<Voice, GetVoiceError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        url.set_query(customisation_id);
        let req = Request::builder()
            .uri(format!("{}/{}", url, voice.id()))
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", self.access_token)).unwrap(),
            )
            .method(Method::GET)
            .body(Body::empty())
            .map_err(|e| GetVoiceError::ConnectionError(e.to_string()))?;
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder().build(https);
        let response = client
            .request(req)
            .await
            .map_err(|e| GetVoiceError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(response).await.unwrap();
                let root: Voice = serde_json::from_reader(body.reader()).unwrap();

                Ok(root)
            }
            StatusCode::NOT_ACCEPTABLE => Err(GetVoiceError::NotAcceptable),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(GetVoiceError::UnsupportedMediaType),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetVoiceError::InternalServerError),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetVoiceError::ServiceUnavailable),
            _ => {
                unreachable!()
            }
        }
    }
}
