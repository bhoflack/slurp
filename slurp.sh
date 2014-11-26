#!/bin/dash

for site in sensors.elex.be sofia.elex.be erfurt.elex.be colo.elex.be
do
    for server in esb-a-uat esb-b-uat
    do
	FQN=$server.$site
	if [ ! -d $FQN ]; then mkdir $FQN; fi
	rsync -avz $FQN:/usr/share/apache-servicemix/data/log/servicemix.log* $FQN
    done
done

if [ ! -d "ewaf.colo.elex.be" ]; then mkdir ewaf.colo.elex.be; fi
rsync -avz ewaf-uat.colo.elex.be:/usr/share/apache-servicemix/data/log/servicemix.log* ewaf.colo.elex.be
