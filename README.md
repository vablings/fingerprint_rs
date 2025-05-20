# fingerprint_rs

fingerprint_rs is a small rust utility that emits a json object with a set of random data to avoid browser fingerprinting. This can be connected up to your web client such as [reqwest](https://docs.rs/reqwest/latest/reqwest/) to circumvent anti-bot measures for web scraping or to stop websites tracking your session with your unique identifier


As of now fingerprint_rs only supports masking for fingerprint_js, i havent spent the time reverse engineering the efforts of the paid version of [fingerprint](https://dev.fingerprint.com/docs/introduction#fingerprint-identification-vs-fingerprintjs)



