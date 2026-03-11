use ext_php_rs::prelude::*;
use wreq_util::Emulation as WreqEmulation;

/// Browser/device profiles for TLS+H2 fingerprint emulation.
/// Use these constants with `ClientBuilder::impersonate()`.
///
/// # Example
/// ```php
/// $client = (new RNet\ClientBuilder())
///     ->impersonate(RNet\Emulation::CHROME_136)
///     ->build();
/// ```
#[php_class]
#[php(name = "RNet\\Emulation")]
pub struct Emulation;

#[php_impl]
impl Emulation {
    // Chrome
    pub const CHROME_100: &'static str = "Chrome100";
    pub const CHROME_101: &'static str = "Chrome101";
    pub const CHROME_104: &'static str = "Chrome104";
    pub const CHROME_105: &'static str = "Chrome105";
    pub const CHROME_106: &'static str = "Chrome106";
    pub const CHROME_107: &'static str = "Chrome107";
    pub const CHROME_108: &'static str = "Chrome108";
    pub const CHROME_109: &'static str = "Chrome109";
    pub const CHROME_110: &'static str = "Chrome110";
    pub const CHROME_114: &'static str = "Chrome114";
    pub const CHROME_116: &'static str = "Chrome116";
    pub const CHROME_117: &'static str = "Chrome117";
    pub const CHROME_118: &'static str = "Chrome118";
    pub const CHROME_119: &'static str = "Chrome119";
    pub const CHROME_120: &'static str = "Chrome120";
    pub const CHROME_123: &'static str = "Chrome123";
    pub const CHROME_124: &'static str = "Chrome124";
    pub const CHROME_126: &'static str = "Chrome126";
    pub const CHROME_127: &'static str = "Chrome127";
    pub const CHROME_128: &'static str = "Chrome128";
    pub const CHROME_129: &'static str = "Chrome129";
    pub const CHROME_130: &'static str = "Chrome130";
    pub const CHROME_131: &'static str = "Chrome131";
    pub const CHROME_132: &'static str = "Chrome132";
    pub const CHROME_133: &'static str = "Chrome133";
    pub const CHROME_134: &'static str = "Chrome134";
    pub const CHROME_135: &'static str = "Chrome135";
    pub const CHROME_136: &'static str = "Chrome136";
    pub const CHROME_137: &'static str = "Chrome137";
    pub const CHROME_138: &'static str = "Chrome138";

    // Edge
    pub const EDGE_101: &'static str = "Edge101";
    pub const EDGE_122: &'static str = "Edge122";
    pub const EDGE_127: &'static str = "Edge127";
    pub const EDGE_131: &'static str = "Edge131";
    pub const EDGE_134: &'static str = "Edge134";
    pub const EDGE_135: &'static str = "Edge135";
    pub const EDGE_136: &'static str = "Edge136";
    pub const EDGE_137: &'static str = "Edge137";

    // Opera
    pub const OPERA_116: &'static str = "Opera116";
    pub const OPERA_117: &'static str = "Opera117";
    pub const OPERA_118: &'static str = "Opera118";
    pub const OPERA_119: &'static str = "Opera119";

    // Firefox
    pub const FIREFOX_109: &'static str = "Firefox109";
    pub const FIREFOX_117: &'static str = "Firefox117";
    pub const FIREFOX_128: &'static str = "Firefox128";
    pub const FIREFOX_133: &'static str = "Firefox133";
    pub const FIREFOX_135: &'static str = "Firefox135";
    pub const FIREFOX_136: &'static str = "Firefox136";
    pub const FIREFOX_139: &'static str = "Firefox139";
    pub const FIREFOX_PRIVATE_135: &'static str = "FirefoxPrivate135";
    pub const FIREFOX_ANDROID_135: &'static str = "FirefoxAndroid135";

    // Safari
    pub const SAFARI_15_3: &'static str = "Safari15_3";
    pub const SAFARI_15_5: &'static str = "Safari15_5";
    pub const SAFARI_15_6_1: &'static str = "Safari15_6_1";
    pub const SAFARI_16: &'static str = "Safari16";
    pub const SAFARI_16_5: &'static str = "Safari16_5";
    pub const SAFARI_17_0: &'static str = "Safari17_0";
    pub const SAFARI_17_2_1: &'static str = "Safari17_2_1";
    pub const SAFARI_17_4_1: &'static str = "Safari17_4_1";
    pub const SAFARI_17_5: &'static str = "Safari17_5";
    pub const SAFARI_17_6: &'static str = "Safari17_6";
    pub const SAFARI_18: &'static str = "Safari18";
    pub const SAFARI_18_2: &'static str = "Safari18_2";
    pub const SAFARI_18_3: &'static str = "Safari18_3";
    pub const SAFARI_26: &'static str = "Safari26";
    pub const SAFARI_IPAD_18: &'static str = "SafariIPad18";
    pub const SAFARI_IOS_16_5: &'static str = "SafariIos16_5";
    pub const SAFARI_IOS_17_2: &'static str = "SafariIos17_2";
    pub const SAFARI_IOS_17_4_1: &'static str = "SafariIos17_4_1";
    pub const SAFARI_IOS_18_1_1: &'static str = "SafariIos18_1_1";
    pub const SAFARI_IOS_26: &'static str = "SafariIos26";

    // OkHttp (Android)
    pub const OK_HTTP_3_9: &'static str = "OkHttp3_9";
    pub const OK_HTTP_3_11: &'static str = "OkHttp3_11";
    pub const OK_HTTP_3_13: &'static str = "OkHttp3_13";
    pub const OK_HTTP_3_14: &'static str = "OkHttp3_14";
    pub const OK_HTTP_4_9: &'static str = "OkHttp4_9";
    pub const OK_HTTP_4_10: &'static str = "OkHttp4_10";
    pub const OK_HTTP_4_12: &'static str = "OkHttp4_12";
    pub const OK_HTTP_5: &'static str = "OkHttp5";
}

/// Parse a string emulation name into a wreq_util Emulation value.
pub fn parse_emulation(s: &str) -> Option<WreqEmulation> {
    let emulation = match s {
        "Chrome100" => WreqEmulation::Chrome100,
        "Chrome101" => WreqEmulation::Chrome101,
        "Chrome104" => WreqEmulation::Chrome104,
        "Chrome105" => WreqEmulation::Chrome105,
        "Chrome106" => WreqEmulation::Chrome106,
        "Chrome107" => WreqEmulation::Chrome107,
        "Chrome108" => WreqEmulation::Chrome108,
        "Chrome109" => WreqEmulation::Chrome109,
        "Chrome110" => WreqEmulation::Chrome110,
        "Chrome114" => WreqEmulation::Chrome114,
        "Chrome116" => WreqEmulation::Chrome116,
        "Chrome117" => WreqEmulation::Chrome117,
        "Chrome118" => WreqEmulation::Chrome118,
        "Chrome119" => WreqEmulation::Chrome119,
        "Chrome120" => WreqEmulation::Chrome120,
        "Chrome123" => WreqEmulation::Chrome123,
        "Chrome124" => WreqEmulation::Chrome124,
        "Chrome126" => WreqEmulation::Chrome126,
        "Chrome127" => WreqEmulation::Chrome127,
        "Chrome128" => WreqEmulation::Chrome128,
        "Chrome129" => WreqEmulation::Chrome129,
        "Chrome130" => WreqEmulation::Chrome130,
        "Chrome131" => WreqEmulation::Chrome131,
        "Chrome132" => WreqEmulation::Chrome132,
        "Chrome133" => WreqEmulation::Chrome133,
        "Chrome134" => WreqEmulation::Chrome134,
        "Chrome135" => WreqEmulation::Chrome135,
        "Chrome136" => WreqEmulation::Chrome136,
        "Chrome137" => WreqEmulation::Chrome137,
        "Chrome138" => WreqEmulation::Chrome138,
        "Edge101" => WreqEmulation::Edge101,
        "Edge122" => WreqEmulation::Edge122,
        "Edge127" => WreqEmulation::Edge127,
        "Edge131" => WreqEmulation::Edge131,
        "Edge134" => WreqEmulation::Edge134,
        "Edge135" => WreqEmulation::Edge135,
        "Edge136" => WreqEmulation::Edge136,
        "Edge137" => WreqEmulation::Edge137,
        "Opera116" => WreqEmulation::Opera116,
        "Opera117" => WreqEmulation::Opera117,
        "Opera118" => WreqEmulation::Opera118,
        "Opera119" => WreqEmulation::Opera119,
        "Firefox109" => WreqEmulation::Firefox109,
        "Firefox117" => WreqEmulation::Firefox117,
        "Firefox128" => WreqEmulation::Firefox128,
        "Firefox133" => WreqEmulation::Firefox133,
        "Firefox135" => WreqEmulation::Firefox135,
        "Firefox136" => WreqEmulation::Firefox136,
        "Firefox139" => WreqEmulation::Firefox139,
        "FirefoxPrivate135" => WreqEmulation::FirefoxPrivate135,
        "FirefoxAndroid135" => WreqEmulation::FirefoxAndroid135,
        "Safari15_3" => WreqEmulation::Safari15_3,
        "Safari15_5" => WreqEmulation::Safari15_5,
        "Safari15_6_1" => WreqEmulation::Safari15_6_1,
        "Safari16" => WreqEmulation::Safari16,
        "Safari16_5" => WreqEmulation::Safari16_5,
        "Safari17_0" => WreqEmulation::Safari17_0,
        "Safari17_2_1" => WreqEmulation::Safari17_2_1,
        "Safari17_4_1" => WreqEmulation::Safari17_4_1,
        "Safari17_5" => WreqEmulation::Safari17_5,
        "Safari17_6" => WreqEmulation::Safari17_6,
        "Safari18" => WreqEmulation::Safari18,
        "Safari18_2" => WreqEmulation::Safari18_2,
        "Safari18_3" => WreqEmulation::Safari18_3,
        "Safari26" => WreqEmulation::Safari26,
        "SafariIPad18" => WreqEmulation::SafariIPad18,
        "SafariIos16_5" => WreqEmulation::SafariIos16_5,
        "SafariIos17_2" => WreqEmulation::SafariIos17_2,
        "SafariIos17_4_1" => WreqEmulation::SafariIos17_4_1,
        "SafariIos18_1_1" => WreqEmulation::SafariIos18_1_1,
        "SafariIos26" => WreqEmulation::SafariIos26,
        "OkHttp3_9" => WreqEmulation::OkHttp3_9,
        "OkHttp3_11" => WreqEmulation::OkHttp3_11,
        "OkHttp3_13" => WreqEmulation::OkHttp3_13,
        "OkHttp3_14" => WreqEmulation::OkHttp3_14,
        "OkHttp4_9" => WreqEmulation::OkHttp4_9,
        "OkHttp4_10" => WreqEmulation::OkHttp4_10,
        "OkHttp4_12" => WreqEmulation::OkHttp4_12,
        "OkHttp5" => WreqEmulation::OkHttp5,
        _ => return None,
    };
    Some(emulation)
}
