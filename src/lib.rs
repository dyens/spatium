#![deny(missing_docs)]
//! # SPATIUM
//!
//! SPATIUM is a library for calcuting distances beetween sequences.
//! ## Algorithms:
//!
//! ### Edit based:
//! - [Hamming](edit_based::hamming::Hamming)
//! - [Levenshtein](edit_based::levenshtein::Levenshtein)
//! - [Damerau-Levenshtein](edit_based::damerau_levenshtein::DamerauLevenshtein)
//!
//! # References:
//!
//! ## Distance Libraries:
//!
//! ### Python libs:
//! - [python textdistance](https://github.com/life4/textdistance)
//! - [python abydos](https://github.com/chrislit/abydos)
//! - [python Distance](https://github.com/doukremt/distance)
//! - [python jellyfish](https://github.com/jamesturk/jellyfish)
//! - [python py_stringmatching](https://github.com/anhaidgroup/py_stringmatching)
//! - [python pylev](https://github.com/toastdriven/pylev)
//! - [python python-Levenshtein](https://github.com/ztane/python-Levenshtein)
//! - [python pyxDamerauLevenshtein](https://github.com/gfairchild/pyxDamerauLevenshtein)

//! ### Large libraries
//! - https://github.com/rockymadden/stringmetric
//! - https://github.com/smashedtoatoms/the_fuzz
//! - https://github.com/howardjp/phonics
//! - https://github.com/MrPowers/spark-stringmetric
//! - https://github.com/n7v/phonetic
//! - https://github.com/wolfpaulus/phonetic-alg-compare
//! - https://github.com/kolipass/phonetic-lib
//! - https://github.com/DaveJPoole/Phonetics
//! - https://github.com/keerthikoneru/Phonetic-Matching-Tool-Kit-with-State-of-the-Art-Meta-Soundex-Algorithm
//!
//! ### Smaller libraries with gimmick algorithms
//! - https://github.com/eldersantos/phonix (Match Rating)
//! - https://github.com/johnjansen/text (Porter Stemming and White similarity)
//! - https://github.com/mlobl/string-metrics (pure string metrics)
//! - https://github.com/Cortexelus/anaphone-solver (CMU dictionary)
//! - https://github.com/CasualSuperman/phonetics (SIFT3)
//! - https://github.com/ahysing/norphone (Norwegian)
//! - https://github.com/outhwest/pysounds (literary hardness)
//!
//! ### Custom algorithms
//! - https://github.com/ticki/eudex (probability based)
//! - https://github.com/lingz/fast_fuzzy_search
//! - https://github.com/jze/phonet4java (Phonet)
//! - https://github.com/ViniciusMRosa/phonetic-algorithms (Portugese)
//! - https://github.com/flezzfx/phonetizz (Phonetizz)
//! - https://github.com/olsgaard/phonetic_search (Phonix)
//!
//! ### Also for long lists
//!
//! - https://github.com/ecomp-shONgit/string-distance
//! - https://github.com/vickumar1981/stringdistance
//! - https://github.com/luozhouyang/python-string-similarity
//! - https://github.com/markvanderloo/stringdist
//! - https://github.com/feature23/StringSimilarity.NET
//! - https://github.com/matthieugomez/StringDistances.jl
//! - https://github.com/dexyk/stringosim
//! - https://github.com/StefH/SimMetrics.Net
//! - https://github.com/ColinFay/tidystringdist
//! - https://github.com/miku/stardust
//! - https://github.com/tdebatty/java-string-similarity
//!
//! - And some of the repos have special algorithms
//!
//! - https://github.com/timoxley/sift (SIFT1)
//! - https://github.com/PrismaPhonic/sift4-rs (SIFT4)
//! - https://github.com/auguscl/Dist4Seq (BSMpre/PURDUEpre)
//! - https://github.com/RThevenoux/ipa-distance (IPA linguistic distance)
//! - https://github.com/ferreirafabio/minimum-edit-distance-py (Wagner-Fischer)
//! - https://github.com/rahulpedduri/LCS (LCS)
//! - https://github.com/NickRimmer/StringCompare (Tanimoto coefficient)
//! - https://github.com/xrash/smetrics (Ukkonen)
//! - https://github.com/hakanozbay/ozbay-metric (Longest Common Substring Distance)
//! - https://github.com/mbrlabs/distance (SIFT3)
//! - https://github.com/ychantit/fuzzymatch_hiveUDF (Ngram)
//! - https://github.com/MailOnline/s-metric (naive brute force)
//! - https://github.com/a1trl9/flatwhite (ged/led)
//! - https://github.com/winkjs/wink-distance (vector-based distances)
//! - https://github.com/ianozsvald/string_distance_metrics (Title string lengths and Uni/bi/trigram distances)
//! - https://github.com/accidental-bebop/BkStringMatch (q-gram distance)
//! - https://github.com/Andreshk/ApproximateStringMatching ("unique")
//! - https://github.com/blester125/string_distance (brew and ratcliff_obershelft)

/// # Edit Based algorithms
pub mod edit_based;

/// # Library error class
pub mod error;

/// # Normalize distance
mod normalize;
