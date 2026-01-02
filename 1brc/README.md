# 1BRC

- https://github.com/gunnarmorling/1brc
- Text file with `Some weather station name;[temp]\n`
- Crunch the test file, answer with min/max/avg for each station
- the text is stored on RAMDISK

## Machine

See [MACHINE]().

## Set up the test environment and run the fastest solution

```
$ git clone https://github.com/gunnarmorling/1brc
$ cd 1brc
$ mkdir /media/measurements
$ sudo mount -t tmpfs -o size=16G tmpfs /media/measurements
$ ln -sf /media/measurements/measurements.txt measurements.txt
$ sdk use java 21.0.2-graal
$ ./create_measurements.sh 1000000000

$ head /media/measurements/measurements.txt
...
Canberra;2.4
Prague;10.1
Madrid;12.2
Bissau;31.1
Tabriz;-5.0
Barcelona;21.6
Hat Yai;3.7
Tbilisi;15.4
Accra;45.6
Cairo;29.6

$ ./mvnw clean package
$ ./prepare_thomaswue.sh

$ target/CalculateAverage_thomaswue_image 
{Abha=-38.2/18.0/67.9, Abidjan=-18.8/26.0/77.3, Abéché=-22.3/29.4/79.4, Accra=-23.4/26.4/76.5, Addis Ababa=-35.8/16.0/66.1, Adelaide=-31.2/17.3/70.9, Aden=-22.4/29.1/80.3, Ahvaz=-22.4/25.4/74.0, Albuquerque=-32.6/14.0/64.8, Alexandra=-37.8/11.0/58.6, Alexandria=-27.9/20.0/68.8, Algiers=-30.8/18.2/67.8, Alice Springs=-28.3/21.0/69.7, Almaty=-38.4/10.0/61.4, Amsterdam=-39.5/10.2/61.1, Anadyr=-60.3/-6.9/42.7, Anchorage=-45.4/2.8/52.4, Andorra la Vella=-41.2/9.8/61.2, Ankara=-39.0/12.0/63.1, Antananarivo=-34.7/17.9/67.6, Antsiranana=-23.6/25.2/75.1, Arkhangelsk=-54.9/1.3/52.3, Ashgabat=-39.4/17.1/68.4, Asmara=-34.3/15.6/62.2, Assab=-15.5/30.5/79.6, Astana=-47.3/3.5/50.3, Athens=-30.7/19.2/67.5, Atlanta=-30.4/17.0/69.2, Auckland=-39.9/15.2/62.7, Austin=-29.1/20.7/71.8, Baghdad=-27.9/22.8/72.6, Baguio=-30.7/19.5/72.4, Baku=-34.7/15.1/64.9, Baltimore=-39.1/13.1/62.0, Bamako=-22.7/27.8/76.7, Bangkok=-25.5/28.6/78.2, Bangui=-24.3/26.0/73.5, Banjul=-23.4/26.0/75.2, Barcelona=-31.2/18.2/67.2, Bata=-29.3/25.1/75.9, Batumi=-37.5/14.0/63.8, Beijing=-41.0/12.9/63.9, Beirut=-26.9/20.9/71.1, Belgrade=-41.8/12.5/61.3, Belize City=-21.2/26.7/81.2, Benghazi=-31.5/19.9/70.8, Bergen=-39.9/7.7/57.3, Berlin=-39.0/10.3/63.5, Bilbao=-35.3/14.7/65.6, Birao=-24.3/26.5/75.6, Bishkek=-37.1/11.3/66.9, Bissau=-27.5/27.0/77.4, Blantyre=-34.4/22.2/74.1, Bloemfontein=-32.5/15.6/63.5, Boise=-38.7/11.4/60.5, Bordeaux=-37.5/14.2/70.9, Bosaso=-16.9/30.0/78.3, Boston=-37.2/10.9/63.6, Bouaké=-25.0/26.0/76.5, Bratislava=-39.9/10.5/60.2, Brazzaville=-22.3/25.0/76.3, Bridgetown=-23.4/27.0/76.6, Brisbane=-26.5/21.4/73.6, Brussels=-39.5/10.5/58.4, Bucharest=-36.8/10.8/58.0, Budapest=-36.8/11.3/59.8, Bujumbura=-30.1/23.8/73.9, Bulawayo=-29.7/18.9/67.2, Burnie=-37.8/13.1/60.1, Busan=-44.4/15.0/68.0, Cabo San Lucas=-23.0/23.9/74.0, Cairns=-29.8/25.0/74.8, Cairo=-28.0/21.4/70.9, Calgary=-44.9/4.4/56.5, Canberra=-36.4/13.1/62.7, Cape Town=-33.0/16.2/70.6, Changsha=-33.4/17.4/66.3, Charlotte=-33.3/16.1/70.2, Chiang Mai=-25.8/25.8/78.1, Chicago=-40.1/9.8/62.3, Chihuahua=-28.8/18.6/67.1, Chittagong=-22.7/25.9/74.4, Chișinău=-41.3/10.2/57.4, Chongqing=-31.9/18.6/66.6, Christchurch=-38.3/12.2/62.0, City of San Marino=-38.6/11.8/60.4, Colombo=-24.6/27.4/78.4, Columbus=-38.8/11.7/64.6, Conakry=-21.4/26.4/76.8, Copenhagen=-43.3/9.1/56.4, Cotonou=-25.2/27.2/81.4, Cracow=-39.2/9.3/59.7, Da Lat=-32.2/17.9/69.0, Da Nang=-25.7/25.8/75.3, Dakar=-26.2/24.0/73.8, Dallas=-33.4/19.0/69.6, Damascus=-31.5/17.0/66.8, Dampier=-21.7/26.4/80.4, Dar es Salaam=-20.1/25.8/79.3, Darwin=-24.4/27.6/78.6, Denpasar=-27.3/23.7/74.5, Denver=-39.4/10.4/63.2, Detroit=-36.9/10.0/56.1, Dhaka=-25.6/25.9/78.8, Dikson=-58.3/-11.1/40.9, Dili=-28.4/26.6/74.0, Djibouti=-17.6/29.9/77.4, Dodoma=-27.8/22.7/76.1, Dolisie=-23.9/24.0/76.4, Douala=-25.2/26.7/76.9, Dubai=-23.6/26.9/76.6, Dublin=-41.5/9.8/60.5, Dunedin=-35.4/11.1/64.1, Durban=-27.7/20.6/76.2, Dushanbe=-33.7/14.7/70.9, Edinburgh=-38.4/9.3/60.9, Edmonton=-46.3/4.2/51.5, El Paso=-33.0/18.1/70.4, Entebbe=-27.9/21.0/70.3, Erbil=-34.1/19.5/68.6, Erzurum=-47.1/5.1/55.0, Fairbanks=-53.7/-2.3/48.1, Fianarantsoa=-30.2/17.9/66.7, Flores,  Petén=-25.6/26.4/77.4, Frankfurt=-36.6/10.6/57.0, Fresno=-31.0/17.9/68.6, Fukuoka=-35.0/17.0/64.6, Gaborone=-26.4/21.0/68.2, Gabès=-31.4/19.5/69.0, Gagnoa=-21.4/26.0/75.9, Gangtok=-33.0/15.2/66.1, Garissa=-21.2/29.3/79.0, Garoua=-21.3/28.3/82.2, George Town=-23.0/27.9/79.4, Ghanzi=-29.3/21.4/73.1, Gjoa Haven=-62.3/-14.4/33.4, Guadalajara=-27.1/20.9/69.5, Guangzhou=-28.0/22.4/78.4, Guatemala City=-30.3/20.4/68.6, Halifax=-53.3/7.5/56.9, Hamburg=-40.3/9.7/61.0, Hamilton=-38.5/13.8/64.8, Hanga Roa=-30.3/20.5/70.2, Hanoi=-28.3/23.6/73.8, Harare=-31.5/18.4/69.8, Harbin=-48.8/5.0/55.1, Hargeisa=-28.1/21.7/72.6, Hat Yai=-26.5/27.0/75.8, Havana=-26.5/25.2/75.5, Helsinki=-48.1/5.9/54.4, Heraklion=-30.0/18.9/69.6, Hiroshima=-33.1/16.3/67.6, Ho Chi Minh City=-24.5/27.4/78.4, Hobart=-38.3/12.7/63.1, Hong Kong=-25.9/23.3/71.4, Honiara=-21.7/26.5/75.6, Honolulu=-24.2/25.4/78.4, Houston=-27.4/20.8/76.1, Ifrane=-41.7/11.4/61.2, Indianapolis=-36.5/11.8/60.9, Iqaluit=-58.7/-9.3/39.4, Irkutsk=-47.9/1.0/49.8, Istanbul=-34.4/13.9/63.3, Jacksonville=-26.1/20.3/71.4, Jakarta=-22.7/26.7/83.8, Jayapura=-21.6/27.0/77.5, Jerusalem=-34.7/18.3/69.5, Johannesburg=-33.2/15.5/72.8, Jos=-25.6/22.8/71.2, Juba=-22.1/27.8/81.2, Kabul=-36.6/12.1/62.9, Kampala=-34.8/20.0/71.0, Kandi=-21.4/27.7/77.1, Kankan=-22.5/26.5/78.1, Kano=-25.4/26.4/75.7, Kansas City=-37.5/12.5/66.1, Karachi=-31.5/26.0/74.3, Karonga=-25.5/24.4/77.9, Kathmandu=-33.5/18.3/70.0, Khartoum=-17.2/29.9/79.9, Kingston=-20.7/27.4/82.1, Kinshasa=-23.1/25.3/75.3, Kolkata=-20.3/26.7/74.9, Kuala Lumpur=-23.8/27.3/81.5, Kumasi=-25.2/26.0/76.9, Kunming=-40.0/15.7/69.0, Kuopio=-49.5/3.4/55.7, Kuwait City=-23.6/25.7/80.3, Kyiv=-49.7/8.4/57.1, Kyoto=-32.8/15.8/62.0, La Ceiba=-21.5/26.2/76.6, La Paz=-29.9/23.7/69.1, Lagos=-21.8/26.8/76.2, Lahore=-26.2/24.3/75.8, Lake Havasu City=-24.1/23.7/70.0, Lake Tekapo=-46.2/8.7/58.1, Las Palmas de Gran Canaria=-27.6/21.2/69.8, Las Vegas=-32.6/20.3/69.9, Launceston=-36.6/13.1/63.2, Lhasa=-42.3/7.6/59.5, Libreville=-22.4/25.9/77.1, Lisbon=-35.6/17.5/65.8, Livingstone=-27.4/21.8/72.4, Ljubljana=-39.1/10.9/61.9, Lodwar=-21.2/29.3/78.5, Lomé=-22.5/26.9/76.0, London=-39.9/11.3/59.0, Los Angeles=-33.4/18.6/70.6, Louisville=-35.9/13.9/62.9, Luanda=-22.4/25.8/81.9, Lubumbashi=-27.3/20.8/72.1, Lusaka=-34.1/19.9/71.3, Luxembourg City=-40.4/9.3/62.6, Lviv=-41.7/7.8/55.6, Lyon=-40.2/12.5/60.3, Madrid=-36.4/15.0/62.0, Mahajanga=-24.7/26.3/73.6, Makassar=-22.1/26.7/75.1, Makurdi=-23.1/26.0/75.5, Malabo=-22.2/26.3/72.6, Malé=-22.3/28.0/79.6, Managua=-23.7/27.3/75.1, Manama=-22.0/26.5/77.1, Mandalay=-23.6/28.0/77.4, Mango=-23.6/28.1/78.4, Manila=-21.0/28.4/78.1, Maputo=-26.5/22.8/70.9, Marrakesh=-38.8/19.6/68.9, Marseille=-35.9/15.8/65.0, Maun=-29.0/22.4/69.7, Medan=-19.5/26.5/78.7, Mek'ele=-29.5/22.7/70.4, Melbourne=-33.8/15.1/62.5, Memphis=-33.3/17.2/66.0, Mexicali=-25.3/23.1/73.9, Mexico City=-30.1/17.5/67.6, Miami=-27.7/24.9/76.9, Milan=-37.1/13.0/63.0, Milwaukee=-37.7/8.9/58.0, Minneapolis=-40.2/7.8/58.1, Minsk=-42.4/6.7/55.2, Mogadishu=-22.8/27.1/77.6, Mombasa=-22.0/26.3/79.1, Monaco=-31.7/16.4/64.6, Moncton=-45.3/6.1/56.2, Monterrey=-29.0/22.3/71.5, Montreal=-43.2/6.8/55.6, Moscow=-41.5/5.8/57.4, Mumbai=-20.5/27.1/80.9, Murmansk=-52.1/0.6/56.9, Muscat=-26.0/28.0/79.0, Mzuzu=-31.2/17.7/68.3, N'Djamena=-20.0/28.3/77.9, Naha=-24.3/23.1/71.4, Nairobi=-30.8/17.8/72.2, Nakhon Ratchasima=-23.5/27.3/77.6, Napier=-34.4/14.6/65.3, Napoli=-35.3/15.9/69.4, Nashville=-32.2/15.4/65.5, Nassau=-22.6/24.6/75.3, Ndola=-31.5/20.3/73.3, New Delhi=-26.0/25.0/72.2, New Orleans=-27.6/20.7/83.1, New York City=-38.5/12.9/63.9, Ngaoundéré=-29.1/22.0/70.6, Niamey=-18.3/29.3/77.2, Nicosia=-30.4/19.7/71.1, Niigata=-36.1/13.9/62.2, Nouadhibou=-30.3/21.3/69.5, Nouakchott=-23.0/25.7/74.8, Novosibirsk=-48.2/1.7/50.7, Nuuk=-48.2/-1.4/47.3, Odesa=-38.1/10.7/61.0, Odienné=-23.5/26.0/74.9, Oklahoma City=-32.7/15.9/65.9, Omaha=-43.2/10.6/59.2, Oranjestad=-23.9/28.1/76.6, Oslo=-44.5/5.7/56.3, Ottawa=-43.9/6.6/56.8, Ouagadougou=-28.2/28.3/76.3, Ouahigouya=-26.6/28.6/77.1, Ouarzazate=-33.0/18.9/68.5, Oulu=-46.2/2.7/52.7, Palembang=-21.7/27.3/82.5, Palermo=-31.8/18.5/67.5, Palm Springs=-30.1/24.5/80.1, Palmerston North=-36.0/13.2/64.8, Panama City=-22.0/28.0/80.5, Parakou=-20.1/26.8/78.7, Paris=-35.4/12.3/62.2, Perth=-34.1/18.7/68.2, Petropavlovsk-Kamchatsky=-45.8/1.9/51.0, Philadelphia=-35.3/13.2/65.7, Phnom Penh=-23.7/28.3/79.8, Phoenix=-25.3/23.9/78.8, Pittsburgh=-37.0/10.8/65.3, Podgorica=-34.1/15.3/69.7, Pointe-Noire=-25.1/26.1/75.7, Pontianak=-27.5/27.7/82.7, Port Moresby=-24.1/26.9/79.4, Port Sudan=-24.0/28.4/77.8, Port Vila=-27.9/24.3/74.3, Port-Gentil=-30.2/26.0/78.7, Portland (OR)=-38.3/12.4/61.7, Porto=-34.6/15.7/64.0, Prague=-41.0/8.4/59.6, Praia=-23.9/24.4/74.9, Pretoria=-30.9/18.2/67.2, Pyongyang=-36.3/10.8/64.3, Rabat=-34.5/17.2/69.2, Rangpur=-24.3/24.4/77.2, Reggane=-20.8/28.3/79.0, Reykjavík=-48.2/4.3/54.5, Riga=-50.4/6.2/58.1, Riyadh=-24.6/26.0/75.6, Rome=-31.9/15.2/67.7, Roseau=-23.3/26.2/76.0, Rostov-on-Don=-41.7/9.9/57.9, Sacramento=-35.2/16.3/66.9, Saint Petersburg=-46.0/5.8/58.4, Saint-Pierre=-44.8/5.7/57.7, Salt Lake City=-38.1/11.6/60.2, San Antonio=-27.3/20.8/69.5, San Diego=-38.2/17.8/67.3, San Francisco=-33.9/14.6/65.0, San Jose=-33.6/16.4/63.8, San José=-24.9/22.6/71.7, San Juan=-18.8/27.2/77.5, San Salvador=-28.8/23.1/75.7, Sana'a=-31.7/20.0/66.4, Santo Domingo=-21.7/25.9/74.4, Sapporo=-47.4/8.9/58.9, Sarajevo=-46.8/10.1/58.7, Saskatoon=-48.3/3.3/52.9, Seattle=-37.3/11.3/64.0, Seoul=-37.2/12.5/63.1, Seville=-30.4/19.2/67.3, Shanghai=-33.2/16.7/72.4, Singapore=-24.0/27.0/82.9, Skopje=-37.6/12.4/59.4, Sochi=-38.4/14.2/63.0, Sofia=-38.7/10.6/59.1, Sokoto=-20.8/28.0/80.2, Split=-33.3/16.1/68.4, St. John's=-41.9/5.0/55.5, St. Louis=-42.6/13.9/62.3, Stockholm=-40.2/6.6/55.8, Surabaya=-23.6/27.1/79.3, Suva=-24.6/25.6/76.1, Suwałki=-41.4/7.2/56.1, Sydney=-35.3/17.7/68.3, Ségou=-19.9/28.0/87.9, Tabora=-28.3/23.0/74.9, Tabriz=-36.7/12.6/60.0, Taipei=-26.4/23.0/72.3, Tallinn=-43.4/6.4/58.5, Tamale=-21.2/27.9/78.1, Tamanrasset=-28.4/21.7/74.3, Tampa=-24.6/22.9/78.3, Tashkent=-36.4/14.8/64.5, Tauranga=-45.2/14.8/71.0, Tbilisi=-36.9/12.9/61.0, Tegucigalpa=-24.9/21.7/70.5, Tehran=-30.3/17.0/64.8, Tel Aviv=-29.9/20.0/67.2, Thessaloniki=-35.8/16.0/64.3, Thiès=-24.9/24.0/77.0, Tijuana=-30.3/17.8/67.8, Timbuktu=-23.0/28.0/77.3, Tirana=-32.6/15.2/64.2, Toamasina=-25.6/23.4/74.3, Tokyo=-32.0/15.4/63.9, Toliara=-24.4/24.1/71.8, Toluca=-39.2/12.4/62.8, Toronto=-38.5/9.4/61.3, Tripoli=-27.1/20.0/68.4, Tromsø=-52.1/2.9/50.4, Tucson=-27.4/20.9/71.4, Tunis=-32.7/18.4/66.4, Ulaanbaatar=-53.1/-0.4/47.5, Upington=-32.7/20.4/74.4, Vaduz=-37.7/10.1/60.2, Valencia=-35.0/18.3/71.2, Valletta=-29.0/18.8/70.2, Vancouver=-41.9/10.4/63.6, Veracruz=-26.9/25.4/74.6, Vienna=-40.6/10.4/58.2, Vientiane=-24.1/25.9/76.8, Villahermosa=-22.5/27.1/77.7, Vilnius=-43.9/6.0/53.1, Virginia Beach=-32.0/15.8/64.9, Vladivostok=-44.6/4.9/56.9, Warsaw=-43.0/8.5/56.1, Washington, D.C.=-37.3/14.6/65.1, Wau=-27.7/27.8/75.2, Wellington=-36.5/12.9/64.3, Whitehorse=-49.6/-0.1/50.3, Wichita=-35.8/13.9/66.8, Willemstad=-27.6/28.0/80.8, Winnipeg=-46.8/3.0/49.4, Wrocław=-41.5/9.6/58.1, Xi'an=-35.7/14.1/61.5, Yakutsk=-58.7/-8.8/42.1, Yangon=-20.9/27.5/79.3, Yaoundé=-23.9/23.8/72.4, Yellowknife=-52.3/-4.3/52.3, Yerevan=-36.2/12.4/61.7, Yinchuan=-40.7/9.0/59.7, Zagreb=-39.9/10.7/63.0, Zanzibar City=-24.6/26.0/72.9, Zürich=-39.3/9.3/57.9, Ürümqi=-42.6/7.4/57.6, İzmir=-31.6/17.9/68.8}

$ hyperfine target/CalculateAverage_thomaswue_image 
Benchmark 1: target/CalculateAverage_thomaswue_image
  Time (mean ± σ):     212.1 ms ±   2.7 ms    [User: 0.6 ms, System: 2.2 ms]
  Range (min … max):   206.7 ms … 217.7 ms    14 runs
```
 

## Performance improvements implemented

1. Perform work in subprocess to skip waiting for huge munmap(!)
2. custom hashmap
3. unsafe code to skip bounds checks
4. multiple scanners per segment for OoOE / IPC
5. PGO

## Results

The current best candidate is as fast on the 8 core run (as in the official 1BRC test), but a bit slower when using all cores.

```
$ taskset -c 0-7 hyperfine --warmup 5 --runs 20 target/CalculateAverage_thomaswue_image ../onebrc/target-pgo-use/release/thomaswue_ported 
Benchmark 1: target/CalculateAverage_thomaswue_image
  Time (mean ± σ):     593.4 ms ±   1.4 ms    [User: 0.5 ms, System: 2.0 ms]
  Range (min … max):   590.4 ms … 595.9 ms    20 runs
 
Benchmark 2: ../onebrc/target-pgo-use/release/thomaswue_ported
  Time (mean ± σ):     595.7 ms ±   3.6 ms    [User: 0.5 ms, System: 0.2 ms]
  Range (min … max):   592.2 ms … 605.1 ms    20 runs
 
Summary
  target/CalculateAverage_thomaswue_image ran
    1.00 ± 0.01 times faster than ../onebrc/target-pgo-use/release/thomaswue_ported

$ hyperfine --warmup 10 --runs 40 target/CalculateAverage_thomaswue_image ../onebrc/target-pgo-use/release/thomaswue_ported 
Benchmark 1: target/CalculateAverage_thomaswue_image
  Time (mean ± σ):     217.1 ms ±   2.2 ms    [User: 0.6 ms, System: 2.1 ms]
  Range (min … max):   213.3 ms … 223.4 ms    40 runs
 
Benchmark 2: ../onebrc/target-pgo-use/release/thomaswue_ported
  Time (mean ± σ):     229.7 ms ±   4.5 ms    [User: 0.7 ms, System: 0.4 ms]
  Range (min … max):   221.1 ms … 238.9 ms    40 runs
 
Summary
  target/CalculateAverage_thomaswue_image ran
    1.06 ± 0.02 times faster than ../onebrc/target-pgo-use/release/thomaswue_ported
```

## WORKNOTES

The old readme grew a bit too large, work notes related to experiements can be found in [WORKNOTES]().
