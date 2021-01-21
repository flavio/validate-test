This repository contains a simple chimera policy that looks at incoming
requests and accepts/rejects them based on the following criteria:

  * If the object comes from a namespace `valid`: accept
  * If the object comes from a namespace not named `valid`: reject without message or code
  * If the object does not have a namespace attribute: reject with error message and code `400`

The repository has two branches, where the code actually lives:

  * wapc: the policy implemented using https://github.com/wapc
  * wasi: a "traditional" chimera policy
