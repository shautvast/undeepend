**currently implementing in rust:** 
* V a sax parser to read xml files (and existing xml binding in rust has trouble reading maven properties)
* V a dom parser to get a generic xml representation
* V a pom reader to get a maven specific representation 
* V to find out what dependencies you have
* V try default localRepository ~/.m2/repository
* load settings.xml
* V search dependency in localRepository
* V download dependency from remote repo's

Why rust and not a maven plugin?
* faster
* more challenges
* run it in docker as a separate step


* report in html
  * list dependencies in descending 'should-I-use-it-score' order (below)
  * drill down to code usage in project

**gradle**
* probably easiest to run gradle itself to get the dependency list 
* maybe should've done that with maven as well...
* but currently it's working rather well (as a POC, it's still missing essential features)

**elaborating**
* deciding if you should ditch a dependency, likely involves other factors:
  * (dependency) project quality, as defined by:
    * date of last commit
    * date of highest version on mavencentral
    * java version in bytecode (pre/post java11, I would say)
    * nr of collaborators
    * nr of issues (ratio open/solved vs total)
    * nr of superseded transitive dependencies
    * reported vulnerabilities
    * in some weighted sum(s), yielding a 'should-I-use-it score'
  * and replaceability score: how much work to replace it
    * how many occurrences of usage?c
    * lib or framework?
* this is going to be a large database, 
* incrementally populated with data
* what stack?

**Another idea**
* compute amount of (dependency) code that is reachable from the application
  * count references (traverse all)
  * what to do with dynamically loaded code?
  