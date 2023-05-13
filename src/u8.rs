use lazy_static::lazy_static;

lazy_static! {
	/// A static lookup table of all odd numbers in the u8 range.
	pub static ref ODDS: [u8; 128] = [
	1,3,5,7,9,11,13,15,17,19,21,23,25,27,29,31,33,35,37,39,41,43,45,47,49,51,53,55,57,59,61,63,65,
		67,69,71,73,75,77,79,81,83,85,87,89,91,93,95,97,99,101,103,105,107,109,111,113,115,117,119,
		121,123,125,127,129,131,133,135,137,139,141,143,145,147,149,151,153,155,157,159,161,163,
		165,167,169,171,173,175,177,179,181,183,185,187,189,191,193,195,197,199,201,203,205,207,
		209,211,213,215,217,219,221,223,225,227,229,231,233,235,237,239,241,243,245,247,249,251,253,255
	];
}

/// Returns true if the given number is odd.
pub fn is_odd(num: u8) -> bool {
    if num == 0u8 {
        false
    } else if num == 1u8 {
        true
    } else if num == 2u8 {
        false
    } else if num == 3u8 {
        true
    } else if num == 4u8 {
        false
    } else if num == 5u8 {
        true
    } else if num == 6u8 {
        false
    } else if num == 7u8 {
        true
    } else if num == 8u8 {
        false
    } else if num == 9u8 {
        true
    } else if num == 10u8 {
        false
    } else if num == 11u8 {
        true
    } else if num == 12u8 {
        false
    } else if num == 13u8 {
        true
    } else if num == 14u8 {
        false
    } else if num == 15u8 {
        true
    } else if num == 16u8 {
        false
    } else if num == 17u8 {
        true
    } else if num == 18u8 {
        false
    } else if num == 19u8 {
        true
    } else if num == 20u8 {
        false
    } else if num == 21u8 {
        true
    } else if num == 22u8 {
        false
    } else if num == 23u8 {
        true
    } else if num == 24u8 {
        false
    } else if num == 25u8 {
        true
    } else if num == 26u8 {
        false
    } else if num == 27u8 {
        true
    } else if num == 28u8 {
        false
    } else if num == 29u8 {
        true
    } else if num == 30u8 {
        false
    } else if num == 31u8 {
        true
    } else if num == 32u8 {
        false
    } else if num == 33u8 {
        true
    } else if num == 34u8 {
        false
    } else if num == 35u8 {
        true
    } else if num == 36u8 {
        false
    } else if num == 37u8 {
        true
    } else if num == 38u8 {
        false
    } else if num == 39u8 {
        true
    } else if num == 40u8 {
        false
    } else if num == 41u8 {
        true
    } else if num == 42u8 {
        false
    } else if num == 43u8 {
        true
    } else if num == 44u8 {
        false
    } else if num == 45u8 {
        true
    } else if num == 46u8 {
        false
    } else if num == 47u8 {
        true
    } else if num == 48u8 {
        false
    } else if num == 49u8 {
        true
    } else if num == 50u8 {
        false
    } else if num == 51u8 {
        true
    } else if num == 52u8 {
        false
    } else if num == 53u8 {
        true
    } else if num == 54u8 {
        false
    } else if num == 55u8 {
        true
    } else if num == 56u8 {
        false
    } else if num == 57u8 {
        true
    } else if num == 58u8 {
        false
    } else if num == 59u8 {
        true
    } else if num == 60u8 {
        false
    } else if num == 61u8 {
        true
    } else if num == 62u8 {
        false
    } else if num == 63u8 {
        true
    } else if num == 64u8 {
        false
    } else if num == 65u8 {
        true
    } else if num == 66u8 {
        false
    } else if num == 67u8 {
        true
    } else if num == 68u8 {
        false
    } else if num == 69u8 {
        true
    } else if num == 70u8 {
        false
    } else if num == 71u8 {
        true
    } else if num == 72u8 {
        false
    } else if num == 73u8 {
        true
    } else if num == 74u8 {
        false
    } else if num == 75u8 {
        true
    } else if num == 76u8 {
        false
    } else if num == 77u8 {
        true
    } else if num == 78u8 {
        false
    } else if num == 79u8 {
        true
    } else if num == 80u8 {
        false
    } else if num == 81u8 {
        true
    } else if num == 82u8 {
        false
    } else if num == 83u8 {
        true
    } else if num == 84u8 {
        false
    } else if num == 85u8 {
        true
    } else if num == 86u8 {
        false
    } else if num == 87u8 {
        true
    } else if num == 88u8 {
        false
    } else if num == 89u8 {
        true
    } else if num == 90u8 {
        false
    } else if num == 91u8 {
        true
    } else if num == 92u8 {
        false
    } else if num == 93u8 {
        true
    } else if num == 94u8 {
        false
    } else if num == 95u8 {
        true
    } else if num == 96u8 {
        false
    } else if num == 97u8 {
        true
    } else if num == 98u8 {
        false
    } else if num == 99u8 {
        true
    } else if num == 100u8 {
        false
    } else if num == 101u8 {
        true
    } else if num == 102u8 {
        false
    } else if num == 103u8 {
        true
    } else if num == 104u8 {
        false
    } else if num == 105u8 {
        true
    } else if num == 106u8 {
        false
    } else if num == 107u8 {
        true
    } else if num == 108u8 {
        false
    } else if num == 109u8 {
        true
    } else if num == 110u8 {
        false
    } else if num == 111u8 {
        true
    } else if num == 112u8 {
        false
    } else if num == 113u8 {
        true
    } else if num == 114u8 {
        false
    } else if num == 115u8 {
        true
    } else if num == 116u8 {
        false
    } else if num == 117u8 {
        true
    } else if num == 118u8 {
        false
    } else if num == 119u8 {
        true
    } else if num == 120u8 {
        false
    } else if num == 121u8 {
        true
    } else if num == 122u8 {
        false
    } else if num == 123u8 {
        true
    } else if num == 124u8 {
        false
    } else if num == 125u8 {
        true
    } else if num == 126u8 {
        false
    } else if num == 127u8 {
        true
    } else if num == 128u8 {
        false
    } else if num == 129u8 {
        true
    } else if num == 130u8 {
        false
    } else if num == 131u8 {
        true
    } else if num == 132u8 {
        false
    } else if num == 133u8 {
        true
    } else if num == 134u8 {
        false
    } else if num == 135u8 {
        true
    } else if num == 136u8 {
        false
    } else if num == 137u8 {
        true
    } else if num == 138u8 {
        false
    } else if num == 139u8 {
        true
    } else if num == 140u8 {
        false
    } else if num == 141u8 {
        true
    } else if num == 142u8 {
        false
    } else if num == 143u8 {
        true
    } else if num == 144u8 {
        false
    } else if num == 145u8 {
        true
    } else if num == 146u8 {
        false
    } else if num == 147u8 {
        true
    } else if num == 148u8 {
        false
    } else if num == 149u8 {
        true
    } else if num == 150u8 {
        false
    } else if num == 151u8 {
        true
    } else if num == 152u8 {
        false
    } else if num == 153u8 {
        true
    } else if num == 154u8 {
        false
    } else if num == 155u8 {
        true
    } else if num == 156u8 {
        false
    } else if num == 157u8 {
        true
    } else if num == 158u8 {
        false
    } else if num == 159u8 {
        true
    } else if num == 160u8 {
        false
    } else if num == 161u8 {
        true
    } else if num == 162u8 {
        false
    } else if num == 163u8 {
        true
    } else if num == 164u8 {
        false
    } else if num == 165u8 {
        true
    } else if num == 166u8 {
        false
    } else if num == 167u8 {
        true
    } else if num == 168u8 {
        false
    } else if num == 169u8 {
        true
    } else if num == 170u8 {
        false
    } else if num == 171u8 {
        true
    } else if num == 172u8 {
        false
    } else if num == 173u8 {
        true
    } else if num == 174u8 {
        false
    } else if num == 175u8 {
        true
    } else if num == 176u8 {
        false
    } else if num == 177u8 {
        true
    } else if num == 178u8 {
        false
    } else if num == 179u8 {
        true
    } else if num == 180u8 {
        false
    } else if num == 181u8 {
        true
    } else if num == 182u8 {
        false
    } else if num == 183u8 {
        true
    } else if num == 184u8 {
        false
    } else if num == 185u8 {
        true
    } else if num == 186u8 {
        false
    } else if num == 187u8 {
        true
    } else if num == 188u8 {
        false
    } else if num == 189u8 {
        true
    } else if num == 190u8 {
        false
    } else if num == 191u8 {
        true
    } else if num == 192u8 {
        false
    } else if num == 193u8 {
        true
    } else if num == 194u8 {
        false
    } else if num == 195u8 {
        true
    } else if num == 196u8 {
        false
    } else if num == 197u8 {
        true
    } else if num == 198u8 {
        false
    } else if num == 199u8 {
        true
    } else if num == 200u8 {
        false
    } else if num == 201u8 {
        true
    } else if num == 202u8 {
        false
    } else if num == 203u8 {
        true
    } else if num == 204u8 {
        false
    } else if num == 205u8 {
        true
    } else if num == 206u8 {
        false
    } else if num == 207u8 {
        true
    } else if num == 208u8 {
        false
    } else if num == 209u8 {
        true
    } else if num == 210u8 {
        false
    } else if num == 211u8 {
        true
    } else if num == 212u8 {
        false
    } else if num == 213u8 {
        true
    } else if num == 214u8 {
        false
    } else if num == 215u8 {
        true
    } else if num == 216u8 {
        false
    } else if num == 217u8 {
        true
    } else if num == 218u8 {
        false
    } else if num == 219u8 {
        true
    } else if num == 220u8 {
        false
    } else if num == 221u8 {
        true
    } else if num == 222u8 {
        false
    } else if num == 223u8 {
        true
    } else if num == 224u8 {
        false
    } else if num == 225u8 {
        true
    } else if num == 226u8 {
        false
    } else if num == 227u8 {
        true
    } else if num == 228u8 {
        false
    } else if num == 229u8 {
        true
    } else if num == 230u8 {
        false
    } else if num == 231u8 {
        true
    } else if num == 232u8 {
        false
    } else if num == 233u8 {
        true
    } else if num == 234u8 {
        false
    } else if num == 235u8 {
        true
    } else if num == 236u8 {
        false
    } else if num == 237u8 {
        true
    } else if num == 238u8 {
        false
    } else if num == 239u8 {
        true
    } else if num == 240u8 {
        false
    } else if num == 241u8 {
        true
    } else if num == 242u8 {
        false
    } else if num == 243u8 {
        true
    } else if num == 244u8 {
        false
    } else if num == 245u8 {
        true
    } else if
    num == 246u8
    {
        false
    }  else if num == 247u8 {
        true
    } else if num == 248u8 {
        false
    } else if num == 249u8 {
        true
    } else if num == 250u8 {
        false
    } else if num == 251u8 {
        true
    } else if num == 252u8 {
        false
    } else if num == 253u8 {
        true
    } else if num == 254u8 {
        false
    } else if num == 255u8 {
        true
    } else { false }
}
