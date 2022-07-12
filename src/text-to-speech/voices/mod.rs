use reqwest::{Method, Request, StatusCode, Url};
use serde::{Deserialize, Serialize};

/// Errors that may be returned in making Voice requests
pub mod errors;

use crate::tts::voices::errors::GetVoiceError;

use self::errors::ListVoicesError;

use super::{customisations::Model, TextToSpeech};
#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
/// Voices available for use in Watson
pub struct Voice {
    #[serde(rename = "url")]
    /// The URI of the voice
    pub url: String,
    #[serde(rename = "gender")]
    /// The gender of the voice: male or female
    pub gender: String,
    #[serde(rename = "name")]
    /// The name of the voice. Use this as the voice identifier in all requests
    pub name: String,
    #[serde(rename = "language")]
    /// The language and region of the voice (for example, en-US)
    pub language: String,
    #[serde(rename = "description")]
    /// A textual description of the voice
    pub description: String,
    #[serde(rename = "customizable")]
    /// If true, the voice can be customized; if false, the voice cannot be customized. (Same as custom_pronunciation; maintained for backward compatibility.)
    pub customisable: bool,
    #[serde(rename = "supported_features")]
    /// Additional service [features](SupportedFeatures) that are supported with the voice
    pub supported_features: Box<SupportedFeatures>,
    #[serde(rename = "customization", skip_serializing_if = "Option::is_none")]
    /// Returns information about a specified custom [model](super::customisations::Model). This field is returned only by the Get a voice method and only when you specify the customization ID of a custom model
    pub customisation: Option<Box<Model>>,
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
/// Additional service features that are supported with the voice
pub struct SupportedFeatures {
    /// If true, the voice can be customized; if false, the voice cannot be customized. (Same as customizable.)
    #[serde(rename = "custom_pronunciation")]
    pub custom_pronunciation: bool,
    /// If true, the voice can be transformed by using the SSML <voice-transformation> element; if false, the voice cannot be transformed. The feature was available only for the now-deprecated standard voices. You cannot use the feature with neural voices.
    #[serde(rename = "voice_transformation")]
    pub voice_transformation: bool,
}

#[derive(Default)]
#[non_exhaustive]
/// All voices that Watson can use
pub enum WatsonVoice {
    /// Arabic - Omar
    ArMsOmar,
    /// Alena - Czech (Czechia)
    CsCzAlena,
    /// Birgit - German (Germany)
    DeDeBirgitV3,
    /// DieterV3 - German (Germany)
    DeDeDieterV3,
    /// ErikaV3 - German (Germany)
    DeDeErikaV3,
    /// Craig - English (Australia)
    EnAuCraig,
    /// Madison - English (Australia)
    EnAuMadison,
    /// Steve - English (Australia)
    EnAuSteve,
    /// CharlotteV3 - English (United Kingdom)
    EnGbCharlotteV3,
    /// JamesV3 - English (United Kingdom)
    EnGbJamesV3,
    /// KateV3 - English (United Kingdom)
    EnGbKateV3,
    /// AllisonV3 - English (United States)
    EnUsAllisonV3,
    /// EmilyV3 - English (United States)
    EnUsEmilyV3,
    /// HenryV3 - English (United States)
    EnUsHenryV3,
    /// KevinV3 - English (United States)
    EnUsKevinV3,
    /// LisaV3 - English (United States)
    EnUsLisaV3,
    #[default]
    /// MichaelV3 - English (United States)
    EnUsMichaelV3,
    /// OliviaV3 - English (United States)
    EnUsOliviaV3,
    /// EnriqueV3 - Spanish (Spain)
    EsEsEnriqueV3,
    /// LauraV3 - Spanish (Spain)
    EsEsLauraV3,
    /// SofiaV3 - Spanish (Latin America)
    EsLaSofiaV3,
    /// SofiaV3 - Spanish (United States)
    EsUsSofiaV3,
    /// LouiseV3 - French (Canada)
    FrCaLouiseV3,
    /// NicolasV3 - French (France)
    FrFrNicolasV3,
    /// ReneeV3 - French (France)
    FrFrReneeV3,
    /// FrancescaV3 - Italian (Italy)
    ItItFrancescaV3,
    /// EmiV3 - Japanese (Japan)
    JaJpEmiV3,
    /// Hyunjun - Koren (South Korea)
    KoKrHyunjun,
    /// SiWoo - Koren (South Korea)
    KoKrSiWoo,
    /// Youngmi - Koren (South Korea)
    KoKrYoungmi,
    /// Yuna - Koren (South Korea)
    KoKrYuna,
    /// Adele - Dutch (Belgium)
    NlBeAdele,
    /// Bram - Dutch (Belgium)
    NlBeBram,
    /// Emma - Dutch (Netherlands)
    NlNlEmma,
    /// Liam - Dutch (Netherlands)
    NlNlLiam,
    /// Isabela - Portuguese (Brazil)
    PtBrIsabelaV3,
    /// Ingrid - Swedish (Sweden)
    SvSeIngrid,
    /// LiNa - Chinese (PRC)
    ZhCnLiNa,
    /// WangWei - Chinese (PRC)
    ZhCnWangWei,
    /// ZhangJing - Chinese (PRC)
    ZhCnZhangJing,
}

impl ToString for WatsonVoice {
    /// The human readable format for Watson Voices
    fn to_string(&self) -> String {
        match &self {
            WatsonVoice::ArMsOmar => "ArMsOmar",
            WatsonVoice::CsCzAlena => "Alena - Czech (Czechia)",
            WatsonVoice::DeDeBirgitV3 => "Birgit - German (Germany)",
            WatsonVoice::DeDeDieterV3 => "DieterV3 - German (Germany)",
            WatsonVoice::DeDeErikaV3 => "ErikaV3 - German (Germany)",
            WatsonVoice::EnAuCraig => "Craig - English (Australia)",
            WatsonVoice::EnAuMadison => "Madison - English (Australia)",
            WatsonVoice::EnAuSteve => "Steve - English (Australia)",
            WatsonVoice::EnGbCharlotteV3 => "CharlotteV3 - English (United Kingdom)",
            WatsonVoice::EnGbJamesV3 => "JamesV3 - English (United Kingdom)",
            WatsonVoice::EnGbKateV3 => "KateV3 - English (United Kingdom)",
            WatsonVoice::EnUsAllisonV3 => "AllisonV3 - English (United States)",
            WatsonVoice::EnUsEmilyV3 => "EmilyV3 - English (United States)",
            WatsonVoice::EnUsHenryV3 => "HenryV3 - English (United States)",
            WatsonVoice::EnUsKevinV3 => "KevinV3 - English (United States)",
            WatsonVoice::EnUsLisaV3 => "LisaV3 - English (United States)",
            WatsonVoice::EnUsMichaelV3 => "MichaelV3 - English (United States)",
            WatsonVoice::EnUsOliviaV3 => "OliviaV3 - English (United States)",
            WatsonVoice::EsEsEnriqueV3 => "EnriqueV3 - Spanish (Spain)",
            WatsonVoice::EsEsLauraV3 => "LauraV3 - Spanish (Spain)",
            WatsonVoice::EsLaSofiaV3 => "SofiaV3 - Spanish (Latin America)",
            WatsonVoice::EsUsSofiaV3 => "SofiaV3 - Spanish (United States)",
            WatsonVoice::FrCaLouiseV3 => "LouiseV3 - French (Canada)",
            WatsonVoice::FrFrNicolasV3 => "NicolasV3 - French (France)",
            WatsonVoice::FrFrReneeV3 => "ReneeV3 - French (France)",
            WatsonVoice::ItItFrancescaV3 => "FrancescaV3 - Italian (Italy)",
            WatsonVoice::JaJpEmiV3 => "EmiV3 - Japanese (Japan)",
            WatsonVoice::KoKrHyunjun => "Hyunjun - Koren (South Korea)",
            WatsonVoice::KoKrSiWoo => "SiWoo - Koren (South Korea)",
            WatsonVoice::KoKrYoungmi => "Youngmi - Koren (South Korea)",
            WatsonVoice::KoKrYuna => "Yuna - Koren (South Korea)",
            WatsonVoice::NlBeAdele => "Adele - Dutch (Belgium)",
            WatsonVoice::NlBeBram => "Bram - Dutch (Belgium)",
            WatsonVoice::NlNlEmma => "Emma - Dutch (Netherlands)",
            WatsonVoice::NlNlLiam => "Liam - Dutch (Netherlands)",
            WatsonVoice::PtBrIsabelaV3 => "Isabela - Portuguese (Brazil)",
            WatsonVoice::SvSeIngrid => "Ingrid - Swedish (Sweden)",
            WatsonVoice::ZhCnLiNa => "LiNa - Chinese (PRC)",
            WatsonVoice::ZhCnWangWei => "WangWei - Chinese (PRC)",
            WatsonVoice::ZhCnZhangJing => "ZhangJing - Chinese (PRC)",
        }
        .to_string()
    }
}

impl WatsonVoice {
    /// The id that the server expects for that voice
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
    /// Lists all voices available for use with the service. The information includes the [`name`], [`language`], [`gender`], and other details about the voice.
    /// The ordering of the list of voices can
    /// change from call to call; do not rely on an alphabetized or static list of voices. To see
    /// information about a specific voice, use [get_voice()](`Self::get_voice()`)
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let voices = tts.list_voices().await?;
    /// println!("Total: {}", voices.len());
    /// # Ok(())
    /// # }
    /// ```
    /// [`name`]: super::voices::Voice::name
    /// [`language`]: super::voices::Voice::language
    /// [`gender`]: super::voices::Voice::gender
    pub async fn list_voices(&self) -> Result<Vec<Voice>, ListVoicesError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| ListVoicesError::ConnectionError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => {
                #[derive(Deserialize)]
                struct Root {
                    voices: Vec<Voice>,
                }
                let root: Root = response.json().await.unwrap();

                Ok(root.voices)
            }
            StatusCode::NOT_ACCEPTABLE => Err(ListVoicesError::NotAcceptable406),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(ListVoicesError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ListVoicesError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(ListVoicesError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }

    fn set_voices_path(uri: &mut Url) {
        uri.set_path("v1/voices");
    }

    /// Returns information about the specified [`Voice`]. The information includes the [`name`], [`language`], [`gender`], and other details about the voice. Specify a customization ID to obtain information for a custom model that is defined for the language of the specified voice. To list information about all available voices, use  [list_voices()](`Self::list_voices()`)
    ///
    /// # Parameters
    ///
    /// * `voice` - The particular [`WatsonVoice`] you want information about
    /// * `customisation_id` - The customization ID (GUID) of a custom model for which information is to be returned. You must make the request with credentials for the instance of the service that owns the custom model. Omit the parameter to see information about the specified voice with no customization
    ///
    /// [`WatsonVoice`]: super::voices::WatsonVoice
    /// [`Voice`]: super::voices::Voice
    /// [`name`]: super::voices::Voice::name
    /// [`language`]: super::voices::Voice::language
    /// [`gender`]: super::voices::Voice::gender
    ///
    /// # Example
    /// ``` no_run
    /// # use ibm_watson::{
    /// #     auth::IamAuthenticator,
    /// #     tts::{voices::WatsonVoice, TextToSpeech},
    /// # };
    /// # async fn foo()-> Result<(), Box<dyn std::error::Error>> {
    /// # let auth = IamAuthenticator::new("api_key").await?;
    /// # let tts = TextToSpeech::new(&auth, "service_url");
    /// let kate = tts.get_voice(WatsonVoice::EnGbKateV3, None).await?;
    /// println!("Gender: {}", kate.gender);
    /// # Ok(())
    /// # }
    /// ```

    pub async fn get_voice(
        &self,
        voice: WatsonVoice,
        customisation_id: Option<&str>,
    ) -> Result<Voice, GetVoiceError> {
        let mut url = Url::parse(self.service_url).unwrap();
        Self::set_voices_path(&mut url);
        url.set_query(customisation_id);
        let req = Request::new(Method::GET, url);
        let client = self.get_client();
        let response = client
            .execute(req)
            .await
            .map_err(|e| GetVoiceError::ConnectionError(e.to_string()))?;
        assert_eq!(response.status(), 200);
        match response.status() {
            StatusCode::OK => {
                let root: Voice = response.json().await.unwrap();

                Ok(root)
            }
            StatusCode::NOT_ACCEPTABLE => Err(GetVoiceError::NotAcceptable406),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Err(GetVoiceError::UnsupportedMediaType415),
            StatusCode::INTERNAL_SERVER_ERROR => Err(GetVoiceError::InternalServerError500),
            StatusCode::SERVICE_UNAVAILABLE => Err(GetVoiceError::ServiceUnavailable503),
            _ => {
                unreachable!()
            }
        }
    }
}
