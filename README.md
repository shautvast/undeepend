currently implementing in rust: 
* a sax parser to read xml files (and existing xml binding in rust has trouble reading maven properties)
* a dom parser to get a generic xml representation
* a pom reader to get a maven specific representation 
* to find out what dependencies you have

Why rust and not a maven plugin?
* faster
* more challenges
* run it in docker as a separate step