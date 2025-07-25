currently implementing in rust: 
* V a sax parser to read xml files (and existing xml binding in rust has trouble reading maven properties)
* V a dom parser to get a generic xml representation
* V a pom reader to get a maven specific representation 
* V to find out what dependencies you have
* try default localRepository ~/.m2/repository
* load settings.xml
* search dependency in localRepository
* download dependency from remote repo's

Why rust and not a maven plugin?
* faster
* more challenges
* run it in docker as a separate step

