# Results 
We just runce once, and take times, but just to have an idea what you can find.  
(To make a properly test we need to run it multiple times and then make an average)

## node-express:
```
Type     Name                              # reqs      # fails |    Avg     Min     Max    Med |   req/s  failures/s
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
GET      /health                             7517     0(0.00%) |    235       0    1026    250 |  125.60        0.00
GET      /note                               7533     0(0.00%) |    251       1    1508    270 |  125.87        0.00
POST     /note                               7631     0(0.00%) |    239       1    1019    260 |  127.51        0.00
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
         Aggregated                         22681     0(0.00%) |    242       0    1508    260 |  378.98        0.00

Response time percentiles (approximated)
Type     Name                                  50%    66%    75%    80%    90%    95%    98%    99%  99.9% 99.99%   100% # reqs
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
GET      /health                               250    310    350    380    460    500    580    670    970   1000   1000   7517
GET      /note                                 270    330    370    410    490    530    650    830   1000   1500   1500   7533
POST     /note                                 260    320    350    390    460    500    570    650    970   1000   1000   7631
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
         Aggregated                            260    320    360    390    470    510    590    740   1000   1300   1500  22681
```

## node-fastify:
```
Type     Name                              # reqs      # fails |    Avg     Min     Max    Med |   req/s  failures/s
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
GET      /health                             7273     0(0.00%) |    245       1    1128    250 |  121.51        0.00
GET      /note                               7292     0(0.00%) |    259       1    1987    250 |  121.83        0.00
POST     /note                               7359     0(0.00%) |    248       1    1063    250 |  122.94        0.00
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
         Aggregated                         21924     0(0.00%) |    251       1    1987    250 |  366.28        0.00

Response time percentiles (approximated)
Type     Name                                  50%    66%    75%    80%    90%    95%    98%    99%  99.9% 99.99%   100% # reqs
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
GET      /health                               250    320    370    400    470    530    590    650   1000   1100   1100   7273
GET      /note                                 250    340    390    420    490    550    670    860   1100   2000   2000   7292
POST     /note                                 250    330    370    400    470    520    580    660   1000   1100   1100   7359
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
         Aggregated                            250    330    380    410    480    530    610    720   1000   1400   2000  21924
```

## dotnet:
```
Type     Name                              # reqs      # fails |    Avg     Min     Max    Med |   req/s  failures/s
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
GET      /health                            17263     0(0.00%) |     75       2     323     74 |  287.93        0.00
GET      /note                              17630     0(0.00%) |    102       2     395     98 |  294.06        0.00
POST     /note                              17141     0(0.00%) |     75       2     299     74 |  285.90        0.00
--------|--------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
         Aggregated                         52034     0(0.00%) |     84       2     395     80 |  867.89        0.00

Response time percentiles (approximated)
Type     Name                                  50%    66%    75%    80%    90%    95%    98%    99%  99.9% 99.99%   100% # reqs
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
GET      /health                                74     87     96    100    120    140    160    170    240    300    320  17263
GET      /note                                  98    120    130    140    170    200    240    260    340    390    400  17630
POST     /note                                  74     87     96    100    120    140    160    170    220    290    300  17141
--------|--------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
         Aggregated                             80     96    110    120    140    170    200    220    300    350    400  52034
```


## rust-rocket:
```
Type     Name                             # reqs      # fails |    Avg     Min     Max    Med |   req/s  failures/s
--------|-------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
GET      /health                           20642     0(0.00%) |     67       1     227     68 |  344.80        0.00
GET      /note                             20780     0(0.00%) |     75       1     256     77 |  347.10        0.00
POST     /note                             20860     0(0.00%) |     68       1     230     71 |  348.44        0.00
--------|-------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
         Aggregated                        62282     0(0.00%) |     70       1     256     72 | 1040.33        0.00

Response time percentiles (approximated)
Type     Name                                 50%    66%    75%    80%    90%    95%    98%    99%  99.9% 99.99%   100% # reqs
--------|-------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
GET      /health                               68     83     92     98    120    130    150    160    210    220    230  20642
GET      /note                                 77     94    100    110    130    150    170    180    220    240    260  20780
POST     /note                                 71     85     95    100    120    130    150    170    210    230    230  20860
--------|-------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
         Aggregated                            72     87     97    100    120    140    160    170    220    230    260  62282
```


## rust-actix:
```
Type     Name                             # reqs      # fails |    Avg     Min     Max    Med |   req/s  failures/s
--------|-------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
GET      /health                           21731     0(0.00%) |     78       1     322     70 |  363.11        0.00
GET      /note                             21244     0(0.00%) |     85       1     354     77 |  354.97        0.00
POST     /note                             21676     0(0.00%) |     82       1     342     74 |  362.19        0.00
--------|-------------------------------|-------|-------------|-------|-------|-------|-------|--------|-----------
         Aggregated                        64651     0(0.00%) |     81       1     354     74 | 1080.28        0.00

Response time percentiles (approximated)
Type     Name                                 50%    66%    75%    80%    90%    95%    98%    99%  99.9% 99.99%   100% # reqs
--------|-------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
GET      /health                               70     95    110    120    160    190    220    240    280    310    320  21731
GET      /note                                 77    100    120    130    170    200    240    250    300    340    350  21244
POST     /note                                 74     99    120    130    160    200    230    250    290    340    340  21676
--------|-------------------------------|--------|------|------|------|------|------|------|------|------|------|------|------
         Aggregated                            74     99    120    130    160    190    230    250    290    330    350  64651
```