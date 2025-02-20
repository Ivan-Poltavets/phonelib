use crate::definitions;

use definitions::Country;

pub const COUNTRIES: [Country; 246] = [
    Country {
        name: "Andorra",
        code: "AD",
        phone_lengths: &[6],
        prefix: 376,
    },
    Country {
        name: "Ascension Island",
        code: "AC",
        phone_lengths: &[4, 5, 6],
        prefix: 247,
    },
    Country {
        name: "United Arab Emirates",
        code: "AE",
        phone_lengths: &[9],
        prefix: 971,
    },
    Country {
        name: "Afghanistan",
        code: "AF",
        phone_lengths: &[9],
        prefix: 93,
    },
    Country {
        name: "Antigua and Barbuda",
        code: "AG",
        phone_lengths: &[10],
        prefix: 1268,
    },
    Country {
        name: "Anguilla",
        code: "AI",
        phone_lengths: &[10],
        prefix: 1264,
    },
    Country {
        name: "Albania",
        code: "AL",
        phone_lengths: &[9],
        prefix: 355,
    },
    Country {
        name: "Armenia",
        code: "AM",
        phone_lengths: &[6, 7, 8],
        prefix: 374,
    },
    Country {
        name: "Angola",
        code: "AO",
        phone_lengths: &[9],
        prefix: 244,
    },
    Country {
        name: "Antarctica",
        code: "AQ",
        phone_lengths: &[4, 5, 6],
        prefix: 672,
    },
    Country {
        name: "Argentina",
        code: "AR",
        phone_lengths: &[6, 7, 8, 10],
        prefix: 54,
    },
    Country {
        name: "American Samoa",
        code: "AS",
        phone_lengths: &[10],
        prefix: 1684,
    },
    Country {
        name: "Austria",
        code: "AT",
        phone_lengths: &[5, 6, 7, 8, 9, 10, 11, 12, 13, 14],
        prefix: 43,
    },
    Country {
        name: "Australia",
        code: "AU",
        phone_lengths: &[9, 10],
        prefix: 61,
    },
    Country {
        name: "Aruba",
        code: "AW",
        phone_lengths: &[7],
        prefix: 297,
    },
    Country {
        name: "Alland Islands",
        code: "AX",
        phone_lengths: &[5, 6, 7, 8, 9, 10],
        prefix: 358,
    },
    Country {
        name: "Azerbaijan",
        code: "AZ",
        phone_lengths: &[9],
        prefix: 994,
    },
    Country {
        name: "Bosnia and Herzegovina",
        code: "BA",
        phone_lengths: &[8],
        prefix: 387,
    },
    Country {
        name: "Barbados",
        code: "BB",
        phone_lengths: &[10],
        prefix: 1246,
    },
    Country {
        name: "Bangladesh",
        code: "BD",
        phone_lengths: &[6, 7, 8, 9, 10, 11],
        prefix: 880,
    },
    Country {
        name: "Belgium",
        code: "BE",
        phone_lengths: &[9, 10],
        prefix: 32,
    },
    Country {
        name: "Burkina Faso",
        code: "BF",
        phone_lengths: &[8],
        prefix: 226,
    },
    Country {
        name: "Bulgaria",
        code: "BG",
        phone_lengths: &[7, 8, 9],
        prefix: 359,
    },
    Country {
        name: "Bahrain",
        code: "BH",
        phone_lengths: &[8],
        prefix: 973,
    },
    Country {
        name: "Palestine",
        code: "PS",
        phone_lengths: &[9],
        prefix: 970,
    },
    Country {
        name: "Israel",
        code: "IL",
        phone_lengths: &[9],
        prefix: 972,
    },
    Country {
        name: "Burundi",
        code: "BI",
        phone_lengths: &[8],
        prefix: 257,
    },
    Country {
        name: "Benin",
        code: "BJ",
        phone_lengths: &[8],
        prefix: 229,
    },
    Country {
        name: "Saint Barthelemy",
        code: "BL",
        phone_lengths: &[9],
        prefix: 590,
    },
    Country {
        name: "Bermuda",
        code: "BM",
        phone_lengths: &[10],
        prefix: 1441,
    },
    Country {
        name: "Brunei Darussalam",
        code: "BN",
        phone_lengths: &[7],
        prefix: 673,
    },
    Country {
        name: "Bolivia",
        code: "BO",
        phone_lengths: &[8, 9],
        prefix: 591,
    },
    Country {
        name: "Brazil",
        code: "BR",
        phone_lengths: &[11],
        prefix: 55,
    },
    Country {
        name: "Bahamas",
        code: "BS",
        phone_lengths: &[10],
        prefix: 1242,
    },
    Country {
        name: "Bhutan",
        code: "BT",
        phone_lengths: &[7, 8],
        prefix: 975,
    },
    Country {
        name: "Bouvet Island",
        code: "BV",
        phone_lengths: &[10],
        prefix: 47,
    },
    Country {
        name: "Botswana",
        code: "BW",
        phone_lengths: &[7, 8],
        prefix: 267,
    },
    Country {
        name: "Belarus",
        code: "BY",
        phone_lengths: &[9],
        prefix: 375,
    },
    Country {
        name: "Belize",
        code: "BZ",
        phone_lengths: &[7],
        prefix: 501,
    },
    Country {
        name: "Cocos (Keeling) Islands",
        code: "CC",
        phone_lengths: &[10],
        prefix: 61,
    },
    Country {
        name: "Congo, Democratic Republic of the",
        code: "CD",
        phone_lengths: &[9],
        prefix: 243,
    },
    Country {
        name: "Central African Republic",
        code: "CF",
        phone_lengths: &[8],
        prefix: 236,
    },
    Country {
        name: "Congo, Republic of the",
        code: "CG",
        phone_lengths: &[9],
        prefix: 242,
    },
    Country {
        name: "Switzerland",
        code: "CH",
        phone_lengths: &[9],
        prefix: 41,
    },
    Country {
        name: "Cote d'Ivoire",
        code: "CI",
        phone_lengths: &[8, 9],
        prefix: 225,
    },
    Country {
        name: "Cook Islands",
        code: "CK",
        phone_lengths: &[5, 7],
        prefix: 682,
    },
    Country {
        name: "Chile",
        code: "CL",
        phone_lengths: &[9],
        prefix: 56,
    },
    Country {
        name: "Cameroon",
        code: "CM",
        phone_lengths: &[9],
        prefix: 237,
    },
    Country {
        name: "China",
        code: "CN",
        phone_lengths: &[11],
        prefix: 86,
    },
    Country {
        name: "Colombia",
        code: "CO",
        phone_lengths: &[10],
        prefix: 57,
    },
    Country {
        name: "Costa Rica",
        code: "CR",
        phone_lengths: &[8],
        prefix: 506,
    },
    Country {
        name: "Cuba",
        code: "CU",
        phone_lengths: &[8],
        prefix: 53,
    },
    Country {
        name: "Cape Verde",
        code: "CV",
        phone_lengths: &[7],
        prefix: 238,
    },
    Country {
        name: "Curacao",
        code: "CW",
        phone_lengths: &[7, 8],
        prefix: 599,
    },
    Country {
        name: "Christmas Island",
        code: "CX",
        phone_lengths: &[6, 7, 8, 9],
        prefix: 61,
    },
    Country {
        name: "Cyprus",
        code: "CY",
        phone_lengths: &[8],
        prefix: 357,
    },
    Country {
        name: "Czech Republic",
        code: "CZ",
        phone_lengths: &[6, 7, 8, 9],
        prefix: 420,
    },
    Country {
        name: "Germany",
        code: "DE",
        phone_lengths: &[6, 7, 8, 9, 10, 11],
        prefix: 49,
    },
    Country {
        name: "Djibouti",
        code: "DJ",
        phone_lengths: &[8],
        prefix: 253,
    },
    Country {
        name: "Denmark",
        code: "DK",
        phone_lengths: &[8],
        prefix: 45,
    },
    Country {
        name: "Dominica",
        code: "DM",
        phone_lengths: &[10],
        prefix: 1767,
    },
    Country {
        name: "Dominican Republic",
        code: "DO",
        phone_lengths: &[10],
        prefix: 1809,
    },
    Country {
        name: "Algeria",
        code: "DZ",
        phone_lengths: &[9],
        prefix: 213,
    },
    Country {
        name: "Ecuador",
        code: "EC",
        phone_lengths: &[9],
        prefix: 593,
    },
    Country {
        name: "Estonia",
        code: "EE",
        phone_lengths: &[8],
        prefix: 372,
    },
    Country {
        name: "Egypt",
        code: "EG",
        phone_lengths: &[10],
        prefix: 20,
    },
    Country {
        name: "Western Sahara",
        code: "EH",
        phone_lengths: &[9],
        prefix: 212,
    },
    Country {
        name: "Eritrea",
        code: "ER",
        phone_lengths: &[7],
        prefix: 291,
    },
    Country {
        name: "Spain",
        code: "ES",
        phone_lengths: &[9],
        prefix: 34,
    },
    Country {
        name: "Ethiopia",
        code: "ET",
        phone_lengths: &[9],
        prefix: 251,
    },
    Country {
        name: "Finland",
        code: "FI",
        phone_lengths: &[9, 11],
        prefix: 358,
    },
    Country {
        name: "Fiji",
        code: "FJ",
        phone_lengths: &[7],
        prefix: 679,
    },
    Country {
        name: "Falkland Islands (Malvinas)",
        code: "FK",
        phone_lengths: &[5],
        prefix: 500,
    },
    Country {
        name: "Micronesia, Federated States of",
        code: "FM",
        phone_lengths: &[7],
        prefix: 691,
    },
    Country {
        name: "Faroe Islands",
        code: "FO",
        phone_lengths: &[5, 6],
        prefix: 298,
    },
    Country {
        name: "France",
        code: "FR",
        phone_lengths: &[9],
        prefix: 33,
    },
    Country {
        name: "Gabon",
        code: "GA",
        phone_lengths: &[8, 9],
        prefix: 241,
    },
    Country {
        name: "United Kingdom",
        code: "GB",
        phone_lengths: &[10],
        prefix: 44,
    },
    Country {
        name: "Grenada",
        code: "GD",
        phone_lengths: &[10],
        prefix: 1473,
    },
    Country {
        name: "Georgia",
        code: "GE",
        phone_lengths: &[9],
        prefix: 995,
    },
    Country {
        name: "French Guiana",
        code: "GF",
        phone_lengths: &[9],
        prefix: 594,
    },
    Country {
        name: "Guernsey",
        code: "GG",
        phone_lengths: &[6],
        prefix: 44,
    },
    Country {
        name: "Ghana",
        code: "GH",
        phone_lengths: &[9],
        prefix: 233,
    },
    Country {
        name: "Gibraltar",
        code: "GI",
        phone_lengths: &[8],
        prefix: 350,
    },
    Country {
        name: "Greenland",
        code: "GL",
        phone_lengths: &[6],
        prefix: 299,
    },
    Country {
        name: "Gambia",
        code: "GM",
        phone_lengths: &[7],
        prefix: 220,
    },
    Country {
        name: "Guinea",
        code: "GN",
        phone_lengths: &[9],
        prefix: 224,
    },
    Country {
        name: "Guadeloupe",
        code: "GP",
        phone_lengths: &[9],
        prefix: 590,
    },
    Country {
        name: "Equatorial Guinea",
        code: "GQ",
        phone_lengths: &[9],
        prefix: 240,
    },
    Country {
        name: "Greece",
        code: "GR",
        phone_lengths: &[10],
        prefix: 30,
    },
    Country {
        name: "South Georgia and the South Sandwich Islands",
        code: "GS",
        phone_lengths: &[5],
        prefix: 500,
    },
    Country {
        name: "Guatemala",
        code: "GT",
        phone_lengths: &[8],
        prefix: 502,
    },
    Country {
        name: "Guam",
        code: "GU",
        phone_lengths: &[10],
        prefix: 1671,
    },
    Country {
        name: "Guinea-Bissau",
        code: "GW",
        phone_lengths: &[9],
        prefix: 245,
    },
    Country {
        name: "Guyana",
        code: "GY",
        phone_lengths: &[7],
        prefix: 592,
    },
    Country {
        name: "Hong Kong",
        code: "HK",
        phone_lengths: &[8],
        prefix: 852,
    },
    Country {
        name: "Heard Island and McDonald Islands",
        code: "HM",
        phone_lengths: &[10],
        prefix: 672,
    },
    Country {
        name: "Honduras",
        code: "HN",
        phone_lengths: &[8],
        prefix: 504,
    },
    Country {
        name: "Croatia",
        code: "HR",
        phone_lengths: &[9],
        prefix: 385,
    },
    Country {
        name: "Haiti",
        code: "HT",
        phone_lengths: &[8],
        prefix: 509,
    },
    Country {
        name: "Hungary",
        code: "HU",
        phone_lengths: &[9],
        prefix: 36,
    },
    Country {
        name: "Indonesia",
        code: "ID",
        phone_lengths: &[9, 10, 11, 12],
        prefix: 62,
    },
    Country {
        name: "Ireland",
        code: "IE",
        phone_lengths: &[9],
        prefix: 353,
    },
    Country {
        name: "Isle of Man",
        code: "IM",
        phone_lengths: &[10],
        prefix: 44,
    },
    Country {
        name: "India",
        code: "IN",
        phone_lengths: &[10],
        prefix: 91,
    },
    Country {
        name: "British Indian Ocean Territory",
        code: "IO",
        phone_lengths: &[7],
        prefix: 246,
    },
    Country {
        name: "Iraq",
        code: "IQ",
        phone_lengths: &[10],
        prefix: 964,
    },
    Country {
        name: "Iran",
        code: "IR",
        phone_lengths: &[11, 10],
        prefix: 98,
    },
    Country {
        name: "Iceland",
        code: "IS",
        phone_lengths: &[7],
        prefix: 354,
    },
    Country {
        name: "Italy",
        code: "IT",
        phone_lengths: &[10],
        prefix: 39,
    },
    Country {
        name: "Jersey",
        code: "JE",
        phone_lengths: &[6],
        prefix: 44,
    },
    Country {
        name: "Jamaica",
        code: "JM",
        phone_lengths: &[10],
        prefix: 1876,
    },
    Country {
        name: "Jordan",
        code: "JO",
        phone_lengths: &[8, 9],
        prefix: 962,
    },
    Country {
        name: "Japan",
        code: "JP",
        phone_lengths: &[10, 11],
        prefix: 81,
    },
    Country {
        name: "Kenya",
        code: "KE",
        phone_lengths: &[9],
        prefix: 254,
    },
    Country {
        name: "Kyrgyzstan",
        code: "KG",
        phone_lengths: &[9],
        prefix: 996,
    },
    Country {
        name: "Cambodia",
        code: "KH",
        phone_lengths: &[8, 9],
        prefix: 855,
    },
    Country {
        name: "Kiribati",
        code: "KI",
        phone_lengths: &[5],
        prefix: 686,
    },
    Country {
        name: "Comoros",
        code: "KM",
        phone_lengths: &[7],
        prefix: 269,
    },
    Country {
        name: "Saint Kitts and Nevis",
        code: "KN",
        phone_lengths: &[10],
        prefix: 1869,
    },
    Country {
        name: "Korea, Democratic People's Republic of",
        code: "KP",
        phone_lengths: &[6, 7, 8, 10, 11],
        prefix: 850,
    },
    Country {
        name: "Korea, Republic of",
        code: "KR",
        phone_lengths: &[7, 8, 9, 10, 11],
        prefix: 82,
    },
    Country {
        name: "Kuwait",
        code: "KW",
        phone_lengths: &[8],
        prefix: 965,
    },
    Country {
        name: "Cayman Islands",
        code: "KY",
        phone_lengths: &[7],
        prefix: 1345,
    },
    Country {
        name: "Kazakhstan",
        code: "KZ",
        phone_lengths: &[10],
        prefix: 7,
    },
    Country {
        name: "Lao People's Democratic Republic",
        code: "LA",
        phone_lengths: &[8, 9],
        prefix: 856,
    },
    Country {
        name: "Lebanon",
        code: "LB",
        phone_lengths: &[7, 8],
        prefix: 961,
    },
    Country {
        name: "Saint Lucia",
        code: "LC",
        phone_lengths: &[7],
        prefix: 1758,
    },
    Country {
        name: "Liechtenstein",
        code: "LI",
        phone_lengths: &[7],
        prefix: 423,
    },
    Country {
        name: "Sri Lanka",
        code: "LK",
        phone_lengths: &[7, 9, 10],
        prefix: 94,
    },
    Country {
        name: "Liberia",
        code: "LR",
        phone_lengths: &[8, 9],
        prefix: 231,
    },
    Country {
        name: "Lesotho",
        code: "LS",
        phone_lengths: &[8],
        prefix: 266,
    },
    Country {
        name: "Lithuania",
        code: "LT",
        phone_lengths: &[8],
        prefix: 370,
    },
    Country {
        name: "Luxembourg",
        code: "LU",
        phone_lengths: &[4, 5, 6, 7, 8, 9],
        prefix: 352,
    },
    Country {
        name: "Latvia",
        code: "LV",
        phone_lengths: &[8],
        prefix: 371,
    },
    Country {
        name: "Libya",
        code: "LY",
        phone_lengths: &[10],
        prefix: 218,
    },
    Country {
        name: "Morocco",
        code: "MA",
        phone_lengths: &[9],
        prefix: 212,
    },
    Country {
        name: "Monaco",
        code: "MC",
        phone_lengths: &[8],
        prefix: 377,
    },
    Country {
        name: "Moldova, Republic of",
        code: "MD",
        phone_lengths: &[8],
        prefix: 373,
    },
    Country {
        name: "Montenegro",
        code: "ME",
        phone_lengths: &[8],
        prefix: 382,
    },
    Country {
        name: "Saint Martin (French part)",
        code: "MF",
        phone_lengths: &[9],
        prefix: 590,
    },
    Country {
        name: "Madagascar",
        code: "MG",
        phone_lengths: &[7, 8, 9],
        prefix: 261,
    },
    Country {
        name: "Marshall Islands",
        code: "MH",
        phone_lengths: &[7],
        prefix: 692,
    },
    Country {
        name: "Macedonia, the Former Yugoslav Republic of",
        code: "MK",
        phone_lengths: &[8],
        prefix: 389,
    },
    Country {
        name: "Mali",
        code: "ML",
        phone_lengths: &[8],
        prefix: 223,
    },
    Country {
        name: "Myanmar",
        code: "MM",
        phone_lengths: &[7, 10],
        prefix: 95,
    },
    Country {
        name: "Mongolia",
        code: "MN",
        phone_lengths: &[8],
        prefix: 976,
    },
    Country {
        name: "Macao",
        code: "MO",
        phone_lengths: &[8],
        prefix: 853,
    },
    Country {
        name: "Northern Mariana Islands",
        code: "MP",
        phone_lengths: &[7],
        prefix: 1670,
    },
    Country {
        name: "Martinique",
        code: "MQ",
        phone_lengths: &[9],
        prefix: 596,
    },
    Country {
        name: "Mauritania",
        code: "MR",
        phone_lengths: &[8],
        prefix: 222,
    },
    Country {
        name: "Montserrat",
        code: "MS",
        phone_lengths: &[10],
        prefix: 1664,
    },
    Country {
        name: "Malta",
        code: "MT",
        phone_lengths: &[8],
        prefix: 356,
    },
    Country {
        name: "Mauritius",
        code: "MU",
        phone_lengths: &[8],
        prefix: 230,
    },
    Country {
        name: "Maldives",
        code: "MV",
        phone_lengths: &[7],
        prefix: 960,
    },
    Country {
        name: "Malawi",
        code: "MW",
        phone_lengths: &[7, 8, 9],
        prefix: 265,
    },
    Country {
        name: "Mexico",
        code: "MX",
        phone_lengths: &[10],
        prefix: 52,
    },
    Country {
        name: "Malaysia",
        code: "MY",
        phone_lengths: &[7, 8, 9, 10],
        prefix: 60,
    },
    Country {
        name: "Mozambique",
        code: "MZ",
        phone_lengths: &[8, 9],
        prefix: 258,
    },
    Country {
        name: "Namibia",
        code: "NA",
        phone_lengths: &[7, 8, 9, 10],
        prefix: 264,
    },
    Country {
        name: "New Caledonia",
        code: "NC",
        phone_lengths: &[6],
        prefix: 687,
    },
    Country {
        name: "Niger",
        code: "NE",
        phone_lengths: &[8],
        prefix: 227,
    },
    Country {
        name: "Norfolk Island",
        code: "NF",
        phone_lengths: &[6],
        prefix: 672,
    },
    Country {
        name: "Nigeria",
        code: "NG",
        phone_lengths: &[7, 8, 9, 10],
        prefix: 234,
    },
    Country {
        name: "Nicaragua",
        code: "NI",
        phone_lengths: &[8],
        prefix: 505,
    },
    Country {
        name: "Netherlands",
        code: "NL",
        phone_lengths: &[9],
        prefix: 31,
    },
    Country {
        name: "Norway",
        code: "NO",
        phone_lengths: &[8],
        prefix: 47,
    },
    Country {
        name: "Nepal",
        code: "NP",
        phone_lengths: &[10],
        prefix: 977,
    },
    Country {
        name: "Nauru",
        code: "NR",
        phone_lengths: &[7],
        prefix: 674,
    },
    Country {
        name: "Niue",
        code: "NU",
        phone_lengths: &[4],
        prefix: 683,
    },
    Country {
        name: "New Zealand",
        code: "NZ",
        phone_lengths: &[8],
        prefix: 64,
    },
    Country {
        name: "Oman",
        code: "OM",
        phone_lengths: &[8],
        prefix: 968,
    },
    Country {
        name: "Panama",
        code: "PA",
        phone_lengths: &[8],
        prefix: 507,
    },
    Country {
        name: "Peru",
        code: "PE",
        phone_lengths: &[9],
        prefix: 51,
    },
    Country {
        name: "French Polynesia",
        code: "PF",
        phone_lengths: &[8],
        prefix: 689,
    },
    Country {
        name: "Papua New Guinea",
        code: "PG",
        phone_lengths: &[7, 8],
        prefix: 675,
    },
    Country {
        name: "Philippines",
        code: "PH",
        phone_lengths: &[10],
        prefix: 63,
    },
    Country {
        name: "Pakistan",
        code: "PK",
        phone_lengths: &[10],
        prefix: 92,
    },
    Country {
        name: "Poland",
        code: "PL",
        phone_lengths: &[9],
        prefix: 48,
    },
    Country {
        name: "Saint Pierre and Miquelon",
        code: "PM",
        phone_lengths: &[6, 8, 9],
        prefix: 508,
    },
    Country {
        name: "Pitcairn",
        code: "PN",
        phone_lengths: &[6],
        prefix: 870,
    },
    Country {
        name: "Portugal",
        code: "PT",
        phone_lengths: &[9],
        prefix: 351,
    },
    Country {
        name: "Palau",
        code: "PW",
        phone_lengths: &[7],
        prefix: 680,
    },
    Country {
        name: "Paraguay",
        code: "PY",
        phone_lengths: &[9],
        prefix: 595,
    },
    Country {
        name: "Qatar",
        code: "QA",
        phone_lengths: &[8],
        prefix: 974,
    },
    Country {
        name: "Reunion",
        code: "RE",
        phone_lengths: &[10],
        prefix: 262,
    },
    Country {
        name: "Romania",
        code: "RO",
        phone_lengths: &[10],
        prefix: 40,
    },
    Country {
        name: "Serbia",
        code: "RS",
        phone_lengths: &[9],
        prefix: 381,
    },
    Country {
        name: "Russian Federation",
        code: "RU",
        phone_lengths: &[10],
        prefix: 7,
    },
    Country {
        name: "Rwanda",
        code: "RW",
        phone_lengths: &[9],
        prefix: 250,
    },
    Country {
        name: "Saudi Arabia",
        code: "SA",
        phone_lengths: &[9],
        prefix: 966,
    },
    Country {
        name: "Solomon Islands",
        code: "SB",
        phone_lengths: &[5, 6, 7],
        prefix: 677,
    },
    Country {
        name: "Seychelles",
        code: "SC",
        phone_lengths: &[7],
        prefix: 248,
    },
    Country {
        name: "Sudan",
        code: "SD",
        phone_lengths: &[7, 9, 10],
        prefix: 249,
    },
    Country {
        name: "Sweden",
        code: "SE",
        phone_lengths: &[7, 8, 9, 10],
        prefix: 46,
    },
    Country {
        name: "Singapore",
        code: "SG",
        phone_lengths: &[8],
        prefix: 65,
    },
    Country {
        name: "Saint Helena",
        code: "SH",
        phone_lengths: &[4, 5],
        prefix: 290,
    },
    Country {
        name: "Slovenia",
        code: "SI",
        phone_lengths: &[8],
        prefix: 386,
    },
    Country {
        name: "Svalbard and Jan Mayen",
        code: "SJ",
        phone_lengths: &[8],
        prefix: 47,
    },
    Country {
        name: "Slovakia",
        code: "SK",
        phone_lengths: &[9],
        prefix: 421,
    },
    Country {
        name: "Sierra Leone",
        code: "SL",
        phone_lengths: &[8],
        prefix: 232,
    },
    Country {
        name: "San Marino",
        code: "SM",
        phone_lengths: &[6, 7, 8, 9, 10],
        prefix: 378,
    },
    Country {
        name: "Senegal",
        code: "SN",
        phone_lengths: &[9],
        prefix: 221,
    },
    Country {
        name: "Somalia",
        code: "SO",
        phone_lengths: &[8, 9],
        prefix: 252,
    },
    Country {
        name: "Suriname",
        code: "SR",
        phone_lengths: &[6, 7],
        prefix: 597,
    },
    Country {
        name: "South Sudan",
        code: "SS",
        phone_lengths: &[7, 9],
        prefix: 211,
    },
    Country {
        name: "Sao Tome and Principe",
        code: "ST",
        phone_lengths: &[7],
        prefix: 239,
    },
    Country {
        name: "El Salvador",
        code: "SV",
        phone_lengths: &[8],
        prefix: 503,
    },
    Country {
        name: "Sint Maarten (Dutch part)",
        code: "SX",
        phone_lengths: &[10],
        prefix: 1721,
    },
    Country {
        name: "Syrian Arab Republic",
        code: "SY",
        phone_lengths: &[7, 8, 9, 10],
        prefix: 963,
    },
    Country {
        name: "Swaziland",
        code: "SZ",
        phone_lengths: &[8],
        prefix: 268,
    },
    Country {
        name: "Turks and Caicos Islands",
        code: "TC",
        phone_lengths: &[10],
        prefix: 1649,
    },
    Country {
        name: "Chad",
        code: "TD",
        phone_lengths: &[6, 8],
        prefix: 235,
    },
    Country {
        name: "French Southern Territories",
        code: "TF",
        phone_lengths: &[10],
        prefix: 262,
    },
    Country {
        name: "Togo",
        code: "TG",
        phone_lengths: &[8],
        prefix: 228,
    },
    Country {
        name: "Thailand",
        code: "TH",
        phone_lengths: &[9],
        prefix: 66,
    },
    Country {
        name: "Tajikistan",
        code: "TJ",
        phone_lengths: &[9],
        prefix: 992,
    },
    Country {
        name: "Tokelau",
        code: "TK",
        phone_lengths: &[4, 5],
        prefix: 690,
    },
    Country {
        name: "Timor-Leste",
        code: "TL",
        phone_lengths: &[8],
        prefix: 670,
    },
    Country {
        name: "Turkmenistan",
        code: "TM",
        phone_lengths: &[8, 9],
        prefix: 993,
    },
    Country {
        name: "Tunisia",
        code: "TN",
        phone_lengths: &[8],
        prefix: 216,
    },
    Country {
        name: "Tonga",
        code: "TO",
        phone_lengths: &[5, 6, 7, 8],
        prefix: 676,
    },
    Country {
        name: "Turkey",
        code: "TR",
        phone_lengths: &[10, 11],
        prefix: 90,
    },
    Country {
        name: "Trinidad and Tobago",
        code: "TT",
        phone_lengths: &[10],
        prefix: 1868,
    },
    Country {
        name: "Tuvalu",
        code: "TV",
        phone_lengths: &[5, 6, 7],
        prefix: 688,
    },
    Country {
        name: "Taiwan",
        code: "TW",
        phone_lengths: &[9],
        prefix: 886,
    },
    Country {
        name: "Tanzania, United Republic of",
        code: "TZ",
        phone_lengths: &[9],
        prefix: 255,
    },
    Country {
        name: "Ukraine",
        code: "UA",
        phone_lengths: &[9],
        prefix: 380,
    },
    Country {
        name: "Uganda",
        code: "UG",
        phone_lengths: &[9],
        prefix: 256,
    },
    Country {
        name: "United States",
        code: "US",
        phone_lengths: &[10],
        prefix: 1,
    },
    Country {
        name: "Uruguay",
        code: "UY",
        phone_lengths: &[8, 9],
        prefix: 598,
    },
    Country {
        name: "Uzbekistan",
        code: "UZ",
        phone_lengths: &[9],
        prefix: 998,
    },
    Country {
        name: "Holy See (Vatican City State)",
        code: "VA",
        phone_lengths: &[5, 6, 7, 8, 9, 10],
        prefix: 379,
    },
    Country {
        name: "Saint Vincent and the Grenadines",
        code: "VC",
        phone_lengths: &[7],
        prefix: 1784,
    },
    Country {
        name: "Venezuela",
        code: "VE",
        phone_lengths: &[10],
        prefix: 58,
    },
    Country {
        name: "Virgin Islands, British",
        code: "VG",
        phone_lengths: &[10],
        prefix: 1284,
    },
    Country {
        name: "Virgin Islands, U.S.",
        code: "VI",
        phone_lengths: &[10],
        prefix: 1340,
    },
    Country {
        name: "Vietnam",
        code: "VN",
        phone_lengths: &[9],
        prefix: 84,
    },
    Country {
        name: "Vanuatu",
        code: "VU",
        phone_lengths: &[5, 6, 7],
        prefix: 678,
    },
    Country {
        name: "Wallis and Futuna",
        code: "WF",
        phone_lengths: &[6],
        prefix: 681,
    },
    Country {
        name: "Samoa",
        code: "WS",
        phone_lengths: &[5, 6, 7],
        prefix: 685,
    },
    Country {
        name: "Yemen",
        code: "YE",
        phone_lengths: &[9],
        prefix: 967,
    },
    Country {
        name: "Mayotte",
        code: "YT",
        phone_lengths: &[9],
        prefix: 262,
    },
    Country {
        name: "South Africa",
        code: "ZA",
        phone_lengths: &[9],
        prefix: 27,
    },
    Country {
        name: "Zambia",
        code: "ZM",
        phone_lengths: &[9],
        prefix: 260,
    },
    Country {
        name: "Zimbabwe",
        code: "ZW",
        phone_lengths: &[9],
        prefix: 263,
    },
];
