/// Tool to leverage Wikidata
pub mod wikidata;
/// Tool to leverage Wikipedia
pub mod wikipedia;

#[cfg(test)]
mod test {
    use indoc::indoc;

    #[tokio::test]
    async fn test_wikidata_sparql_direct() {
        let query = indoc! {
        r#"
        PREFIX wd: <http://www.wikidata.org/entity/>
        PREFIX wdt: <http://www.wikidata.org/prop/direct/>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        
        SELECT ?country ?countryLabel ?capital ?capitalLabel
        WHERE {
          ?country wdt:P31 wd:Q6256 .         # ?country is an instance of a country (Q6256)
          ?country wdt:P36 ?capital .          # ?country has a capital (?capital)
          SERVICE wikibase:label {
            bd:serviceParam wikibase:language "en" .    # Use English labels
            ?country rdfs:label ?countryLabel .
            ?capital rdfs:label ?capitalLabel .
          }
 