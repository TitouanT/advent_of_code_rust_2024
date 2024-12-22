// vim: nowrap
const LUT1: [u32;1000] = [0,66,96,162,272,250,336,490,416,522,740,726,840,910,1092,1200,1280,1360,1476,1558,1120,1470,1056,1288,1776,1500,1716,2052,1736,1972,2040,2418,2304,1782,2788,2660,2376,3108,2964,2652,3040,3362,3276,3354,2992,3240,3312,3760,3936,4018,2900,3774,3328,3392,3888,2750,3248,4332,3596,4012,4200,5002,4712,4410,5120,4810,3696,5628,5304,4692,5460,5964,5760,5840,6216,6000,6080,5390,5772,5846,4800,6156,5412,5478,6384,5610,5676,6438,4576,5340,6480,7644,7176,6696,7896,7410,6912,7954,7448,5742,5800,9494,7752,8446,9984,8190,8904,10486,8640,9374,6380,5550,6048,6102,7068,7360,7424,7488,7788,7854,7440,9196,6588,7626,9920,8250,9072,10414,8704,9546,8840,10218,9504,7182,10988,10260,8976,11508,10764,9452,9800,10716,10224,10296,8928,9570,9636,10878,11248,11324,10800,13288,11856,11934,13244,9920,11232,14130,12008,13038,12480,14490,13608,12714,14432,13530,10624,15364,14448,12844,12240,13338,12728,12802,13572,12950,13024,11328,12104,12172,13320,16290,14560,14640,16560,14800,14880,16456,12408,13986,15200,17572,16512,15440,17848,16770,15680,17730,16632,13134,9600,16884,13332,14616,17544,13940,15244,18216,14560,15884,14700,13082,13992,14058,15836,16340,16416,16492,17004,17082,10560,13702,8880,10704,14784,11700,13108,15436,12312,13740,14260,16632,15312,11184,17784,16450,14160,18486,17136,14818,17760,19280,18392,18468,16104,17150,17220,19266,19840,19920,15000,19076,16632,16698,18796,13260,15360,20046,16512,18130,18720,21924,20436,18936,21648,20140,15428,22962,21440,18830,20520,22222,21216,21294,22468,21450,21528,18836,20016,20088,17360,21918,19176,19244,22152,19380,19448,21812,15552,17918,21460,25026,23360,21682,25284,23600,21904,24948,23244,17940,13200,24080,18724,20604,24928,19520,21420,25788,20328,22248,19220,16794,18096,18154,20724,21420,21488,21556,22260,22330,17920,22470,15456,18088,23976,19500,21516,24852,20336,22372,14520,17874,15936,9990,19372,17420,14112,20220,18252,14916,22440,24552,23256,23324,19952,21390,21452,24290,25056,25128,21000,26676,23232,23298,26196,18460,21360,27846,22912,25130,20160,24548,22444,20328,24024,21900,15372,25690,23552,19926,25160,27454,26040,26110,27676,26250,26320,22620,24192,24256,23560,29718,25976,26044,29952,26180,26248,29412,20952,24118,22620,27370,25088,22794,27580,25280,22968,26996,24676,17556,24000,38496,31356,33852,39592,32400,34916,40700,33456,35992,30340,27126,28840,28910,32292,33200,33280,33360,34276,34358,29400,35364,26164,29610,37312,31450,34080,38430,32528,35178,32680,37066,34560,26846,39060,36540,32264,40204,37668,33364,26400,29106,27404,27466,23088,24920,24976,28608,29568,29634,28800,36080,31640,31710,35412,25480,29184,37474,31144,33966,32200,37802,35112,32410,37120,34410,26096,39228,36504,31892,33840,36738,34928,35002,36972,35150,35224,30528,32504,32572,35520,43290,38560,38640,43560,38800,38880,42856,32208,36186,39200,45172,42312,39440,45448,42570,39680,44730,41832,32934,25000,43086,34136,37222,44352,35350,38456,45630,36576,39702,37740,33726,35840,35910,40092,41200,41280,41360,42476,42558,33280,40638,29232,33472,42968,35700,38924,44268,36960,40204,37100,42480,39368,29848,44856,41730,36448,46182,43040,37730,38880,42198,40108,40182,34816,37060,37128,41572,42744,42822,27500,36366,30912,30968,35456,23310,27800,37876,30132,33540,35840,42636,39340,36032,41736,38420,28300,44226,40896,35278,43320,46822,44616,44694,47068,44850,44928,39236,41616,41688,35960,45318,39576,39644,45552,39780,39848,44612,31752,36518,43660,50826,47360,43882,51084,47600,44104,50148,46644,35940,27600,49282,38528,42210,50736,39930,43632,52202,41344,45066,40260,35438,37944,38006,42980,44280,44352,44424,45732,45806,37200,45954,32344,37380,48672,40000,43820,50160,41448,45288,37800,44170,40448,29118,46916,43180,36888,48412,44660,38340,40960,44870,42372,42438,36064,38700,38760,43996,45360,45430,37700,48174,41728,41792,47088,32750,38048,49932,40796,44812,30360,38338,34424,30498,37184,33250,21312,40020,36072,29436,45560,49654,47040,47110,49876,47250,47320,40620,43392,43456,42160,53118,46376,46444,53352,46580,46648,52212,37152,42718,40020,48370,44288,40194,48580,44480,40368,47396,43276,30756,43400,68698,56160,60458,70400,57810,62128,72114,59472,63810,53960,48348,51264,51336,57120,58630,58712,58794,60312,60396,51840,62006,46208,52056,65160,55100,59532,66884,56784,61236,56940,64328,60024,46912,67528,63210,55936,69278,64944,57642,56240,60762,57876,57954,50592,53640,53712,59760,61336,61418,54000,66088,58656,58734,64844,48320,54432,68130,57608,62238,59280,68490,64008,59514,67232,62730,49024,70564,66048,58444,47740,52428,49408,49472,52632,49600,49664,41958,45124,45182,51480,64042,56304,56376,64288,56520,56592,62960,45704,52074,56880,66444,61776,57096,66696,62010,57312,65354,60648,46342,41600,70488,56140,61028,72360,57960,62868,74244,59792,64720,61560,55148,58464,58536,65120,66830,66912,66994,68712,68796,54120,65680,47676,54318,69216,57750,62776,71122,59616,64662,59760,68142,63232,48314,71724,66800,58520,73656,68716,60408,63840,68962,65676,65754,57392,60840,60912,67760,69536,69618,56100,69782,61344,61416,68320,49590,56496,71988,60060,65284,61920,72324,67236,62136,70848,65740,50228,74562,69440,60830,64380,69680,66272,66348,69920,66500,66576,57882,61460,61530,45760,59908,51156,51214,60112,51330,51388,58542,39072,46228,58740,69498,64224,58938,69732,64440,59136,68172,62860,46748,43200,75684,59532,65016,77744,61540,67044,79816,63560,69084,61880,54660,58368,58432,65808,67710,67784,67858,69768,69844,57040,69996,49788,57226,73920,61050,66672,76014,63104,68746,57660,67032,61512,44784,70984,65450,56160,73086,67536,58218,63920,69634,65940,66010,56640,60480,60544,68184,70152,70226,58900,74178,64736,64804,72504,51570,59272,76560,63228,69048,59520,71114,65416,59706,69408,63690,46368,73492,67760,58140,64020,69912,66096,66164,70128,66300,66368,56666,60636,60698,58800,74556,64812,64878,74784,65010,65076,73038,51376,59340,47520,59460,53568,47664,59640,53730,47808,57826,51896,33966];
const LUT2: [u64;1000] = [0,80883999266,119883520216,201892339872,323535997072,299708800550,403784679756,566187994890,479534080896,605677019652,951719373800,889723991926,1090765782096,1155823196736,1338912600900,1510577757450,1579480191904,1625822443984,1812693308976,1875632727924,1444840527720,1819179631752,1318718722376,1661566606878,2216908890192,1867359379800,2133301604408,2494022501520,2091442505432,2379451789590,2652532939500,2975099755846,2934277471744,2220815738592,3458295111808,3410392063330,2953802221488,3763438798218,3702711383120,3199952406690,3806877495280,4168651216720,4174266721128,4188184135064,3558895967792,4090371682950,4089835927004,4494920874544,4833848823936,4837158087804,3612101319400,4683806826846,4198073399624,4278805580386,4677890481756,3296796806050,4045553477728,5265158614320,4332273761252,4840953640890,5305065879120,6172145181900,6008290979612,5549535638514,6142141431552,5960251114610,4441631477316,6814875661638,6625904580320,5661454257990,6662035616880,7218883814462,7155885807792,7110173066650,7523907074228,7454047716450,7402371959800,6228067943790,7089977583936,7023848657404,5779362111200,7438987313388,6620038822648,6700771003412,7714505362032,6862235364940,6942967545704,7536601331892,5274874889856,6429540348710,7957598818860,9207626419082,8915528550576,8192171657040,9511174542788,9206252307660,8456435258880,9309183107390,8986224757608,6662447216172,7276088485000,11811535240066,9792372739416,10646012104872,12162372920672,10080383702550,10956090127556,12513210601290,10368394665696,11266168150252,8003697333500,6490497087696,7670442749536,7514321928046,8347714047480,9003825099500,8851549297824,8567390733174,9238707493636,9080468676454,9694363470720,11515804423222,8355303709316,9936722557488,12513526913872,10404838882750,11414894094828,12816273533010,10654555016192,11686677287820,11390436481840,12467523797304,11998423603320,8844290410886,13522682310420,13046501352060,11050141334736,13825428929584,13336423604604,11293894452692,12251889990760,13256253528552,13025498891912,12832990684800,10544480902080,12069613103990,11862651726996,12932792004888,13770918464832,13567802107652,13589159636700,16638949831136,15060851896704,15159936448656,16166915796416,12135590351500,14132726022168,17383601267150,14701385928672,15963986831100,15588284398400,17740867038818,17158542961968,15826791458908,17216715523784,16616355332280,12666872271024,18490837016984,17883396061104,15388983598612,14877294989120,16076733003054,15777364855336,15525226493146,16358780950476,16052551451650,15794450073952,12960924442494,14816490569392,14543936021812,16306991564400,19944701453578,18033388455628,18132473007582,20275276615792,18330642111490,18429726663444,19631254896022,14719296684776,17122341142620,18511087723480,21046618661340,20336050918272,18739697862774,21377193823560,20653801713870,19030988503128,20681054623476,19939626399132,15184985433734,12646602709600,21591022454964,17468108167212,19047788312966,21913276521864,17727535516640,19329282721944,22235530588776,17986962866080,19610777130934,19300370625480,16377522210214,18577928438192,18242186075040,19767437604212,20949551246170,20617653902832,20044551215920,21241870566320,20904010207476,13911262980560,17153708096954,11307069893940,14100962021204,18673130766080,14779206529650,16507277044620,18923217339294,14976262617168,16726400191688,19401266262020,21230407688490,20330563196512,14733292156684,22850130194424,21943204938070,18404608357152,23143080582006,22223330959032,18638565243526,23436030968160,25100779388852,24651449677712,24270310916100,20340374584480,22876958339690,22481366408196,24234357082988,25589477120032,25197730208352,19496407158000,24493320899290,21791897317512,21878373100518,23462285754532,16749767400270,19964320929792,25215505143028,20752998179004,22738558964086,24481955392960,27907766947584,26894698572740,24677676484286,26852644397568,25821539908070,19428918999420,28691329883586,27653144629712,23616495603626,26365534839720,28225357736554,27707414514336,27266645597646,28537815571276,28013010997950,27566279065752,23091326885394,25958344565592,25497159463512,21835976017520,27420809453552,24386170808256,24472646591264,27713558308928,24645598157280,24732073940288,26510535479120,18917384358528,22537846675226,27306796400420,31115556252486,29974244211424,27492620570532,31436335182924,30282198775240,27774114979104,30209224947858,29037052425520,21839273612728,16999006129800,30355357076982,24131649213648,26440328351220,30657902164736,24371367583930,26702113781052,30960447252502,24611085954224,26963899210896,24336390999760,19971346733402,23159665845888,22611754259444,24796317516724,26471886066990,25927819500360,25033225009556,26723999268264,26173969686128,23117448443520,27807460085352,19301246754776,23334174522678,29928270017592,24275671937400,26748320116808,30205383629520,24499755063632,26994470302590,18698906742780,21255677713042,19900664355856,11835830901420,23366690510152,22004596388590,16899338676864,23576570964510,22201652476328,17050225630014,28644350376280,30945945011080,30254501556648,29661193571744,24066292022432,27590788662150,26983027507364,29395329910264,31243904855616,30639988720644,27294970021200,34251616078290,30439475618112,30525951401118,32699406130332,23318303635670,27762883792992,35026985743428,28796795922804,31517925359486,25709748786000,30389309267378,28926016607088,25804243945788,28744775720024,27263446945080,18408208201584,31089585236504,29601175697184,24002810120052,31171793057280,33668462168222,32908405202712,32255467062730,33940713883868,33173795567250,32514894411760,26374977013710,30229907578416,29556553253044,29634538880920,37179104632752,33033749109056,33120224892064,37471853488128,33293176458080,33379652241088,35747655855120,25485920594128,30336409538626,27852227852280,32914736631100,31323200304592,27936826090854,33167279367400,31562918674270,28150084305288,31350758134996,29728361327212,20067964679574,29104353940800,46895303280668,38593469032620,41653814352878,47246140961880,38881479996360,41963892376168,47596978643104,39169490960112,42273970399470,38356035256640,32577243997290,36781924852384,36050292900748,38922616744236,41120089060910,40392302129088,39204664692492,41417342717608,40683592770280,37496136985440,43641740630118,32483977649388,37763966535336,46388014257696,38984767811150,42210019132404,46716231340562,39259954408320,42507272789124,41326825513920,44678361548560,42935314846752,32470054498294,47482071198604,45731943732420,39127160410472,47810288281496,46047336448692,39396383992156,32014789334880,34955145018990,34023502656468,33219940283474,25961988351672,30476312711100,29658297168224,32731826134434,35075770823296,34261600300234,36353863016100,45273042122616,40352985517664,40442262034296,43208059572336,31161173670900,36838581189648,46118401610910,38123329667312,41582828489220,40304621398200,46276878977698,44402310203328,40414735140548,44159778947104,42267174058080,30988265651104,47127557008144,45227871354744,38106737397932,41131344969920,44281527745254,43296024486736,42447584573746,44563575692676,43571211083050,42716808154552,34928592989094,39788103888592,38919247790212,43485310838400,53002217674978,47758754041828,47857838593782,53332792837192,48056007697690,48155092249644,51125246707822,38207536075376,44300660416620,47739120971080,54104134883340,52111130478072,47868761898174,54434710045560,52428881273670,48160052538528,52175046435876,50151181549332,38076923273534,31616506775000,53816429105166,43410843070016,47197229170572,54138683172672,43670270420050,47478723580156,54460937240190,43929697770096,47760217989752,49530320159220,42326302057788,47535864806400,46609032640482,50157637221088,52865533455080,51942546308976,50450386077498,53173488019932,52244537858322,43692185785040,51271093859888,37439493656328,43944256087646,54575968464776,45399786079200,49355352969624,54888426300552,45659213429280,49636847379254,49207047553500,53310637699146,51136846699344,38228448503492,56678911342008,54498039577830,46351019692288,56997332193318,54803636062520,46610447042390,49629524466600,53239274046448,52098065622264,51114881707608,42224512239744,47759297165310,46761660080772,50527048456720,53396995735520,52403203670796,34778157452500,45639515526108,39591188694048,39662911861972,43000698126504,28267674735960,35157555533800,46432740343554,36652432194648,40829946319448,47237865682560,54483352176264,52178039103900,47305193455726,51835281110688,49511931923670,35789885669300,55367623164546,53037193213152,44373822692746,55660573550520,59471141208754,58267062875736,57229992408246,59783599043476,58572659359350,57529625876352,48099984161994,53970946614792,52913459961912,45231664607720,56695694991152,50328905710656,50415381493664,56988443846528,50588333059680,50674808842688,54221896607120,38622993065328,45933535265426,55555206469820,63193449296286,60769700593024,55642061427732,63514228226724,61077655156840,55923555836304,60723593582058,58268984397520,43751588274328,33998012260800,60609865792784,48103486182052,52618871274426,60912410881144,48343204552940,52880656704864,61214955969516,48582922923840,53142442135314,51066903429300,42420689476776,48618165109896,47479163720686,51687080029400,54888431171700,53753274802304,51939622766934,55156179617676,54015060232774,48021340140400,57032236068066,40525482064324,48253701463660,60891761918208,49941327016250,54625892898492,61184510774838,50181045387184,54887678328976,48795877884600,53639397840316,51022738241584,35659077897402,57535862573188,54912122210080,45210741460008,57808114290108,55171549560380,45423999675720,50242871742720,54489467536676,53106145369200,51910792231252,41355457545696,47878155355770,46668349047940,51093049151996,54456451339104,53250490051088,46957317152200,59787416554446,52637381856824,52718114037586,56654451390156,39261852872050,47390769310528,60687880870320,49148898877652,54070990666890,37397813486880,45892104327576,43131622390004,37349081589222,42639788525376,39861270473150,23671661804172,46663420870410,43877822053808,33647790402594,56446219860480,60893633732822,59447441656512,58198201965730,61165885448468,59712832021050,58457629314760,47363022382710,54221897720016,52952241843844,53030227471120,66453990170352,58976484011456,59062959794464,66746739025728,59235911360480,59322387143488,63459016983120,45191529300928,53732098128826,49277018507880,58169010261100,55295037272392,49262647534254,58421552997400,55534755642070,49475905748688,55041507355396,52136673885412,35156659927374,50932619397800,81979071322470,67394565327024,72661616602084,82329909004288,67682576291370,72971694625980,82680746686118,67970587255728,73281772649888,66421426909260,56356254215712,63564879843408,62236946340974,67127411487864,70845454648540,69521366165920,67409459436726,71142708305844,69812656807718,64279091976480,74740368158960,55576852757032,64546921526382,79209722460344,66503427444000,71935384720056,79537939543816,66778614041776,72232638377382,70159494478580,75776989078022,72751505714016,54966628055160,80303779401872,77271215273490,66049518492544,80631996485370,77586607990368,66318742074834,69227966074440,74139856940604,72542869887936,71163796384796,58971945339648,66511004892330,65117478219308,70229938909572,74115244863088,72725563209880,66957387475500,81764253683792,73520536598016,73618303269024,78161217187040,58117069018780,67493046575304,82820110362542,69530480003520,75205174934604,73042761374960,82852992083234,79704476801040,73079376368236,79197838106168,76031286709320,57441251146320,83914167304136,80740535142912,69010977881476,56025881337580,61112056259232,59425665274832,57966171195960,61349846361408,59656594025900,58191136931520,45433479616980,53282182673996,51802272409734,63013362562800,78399658311458,69814236007788,69903512524422,78700809367712,70082065557690,70171342074324,74900314722782,53967043633816,63740439823140,69218806315880,79403495167220,76118246064432,69220053924814,79704646223480,76406572754070,69481920459208,75852034098236,72535924514172,53132240892654,50586410841600,86041835756568,69353577974020,75346670029378,86364089824680,69613005324660,75628164439568,86686343892804,69872432675312,75909658849770,78665802607440,67175403071810,75388910593024,73865776876308,79432522760316,83660989838310,82141701141408,79725271617332,83968944403768,82443692691360,68899216047280,80793796659930,58956444035172,69151286349892,85821751938624,71342520983250,77504793828476,86134209775006,71601948333936,77786288238712,77060093340160,83429642050608,79973414388608,59745398882358,88520996367276,85057687940900,72293754596360,88839417219192,85363284426196,72553181947068,81579350852160,87145056941442,85322925154492,83748651437070,69908804183240,78452745629440,76864018742136,82652760169518,87048490040352,85463608172862,71419919073400,88181264515062,78821577468456,78914091057034,84041293967820,61323308576730,71924059678624,89258788120632,74196221822580,80601232322834,79845397918720,90928857541200,87349597961012,79839226942814,86742732529152,83145436834310,62112263423916,92023625719938,88419249260720,75147455435690,79958678309040,85714247126430,83818290080368,82179174459834,86009474154420,84106655757250,82461577121208,68071502270606,76940665893760,75281134087836,55645051925760,72973526641510,63259834110732,63331557278658,73222017651640,63475003614510,63546726782436,68847688157386,45228279579312,56214149047728,75074465104420,86532382869966,82816389469184,75032926744172,86823737694444,83094919927040,75284997046784,82440154534218,78693300652160,56846479183248,50997018393000,90864374509786,72075323151656,78797414198832,91166919598752,72315041523150,79059199629876,91469464687730,72554759894656,79320985060932,76181773970120,63249178583398,72450598989120,70715296049112,76941353661228,81663275647530,79931817727336,77193896399368,81931024094112,80193603158412,71257472468240,84584040933708,60071534508768,71489833791506,90166647457656,73913163985900,80804435822944,90459396314892,74152882357440,81066221254034,72032010212460,79141488732978,75242392472048,52559114817468,84761034139656,80854857114350,66536562905280,85033285857182,81114284465256,66749821121598,78693261024080,84895813327470,82831567797228,81045124856514,65540312384992,75072166715700,73271270605104,79719323760594,84608508539736,82811457448954,73581085700900,92295593798580,81818620213792,81904564142588,87614739468792,62221600277520,74045808347432,93386884866858,76550066052424,83684075544496,74355623445120,86700187680870,82678059501752,74270293597224,81946718731032,77906554437260,54418118878536,87755803011828,83708557953680,68882403264120,76149352487000,82541767518498,80403696729144,78552411885318,82796788427412,80651856286950,78794608427616,62739568359322,72596644866984,70724943837768,70797185862200,90094401906588,79279001510248,79359733691012,90369919955232,79521198052540,79601930233304,85501442696292,59222458990656,71447364099710,56096720232300,68803442344838,64632280078848,55939122201228,69011727235892,64827740603280,56108122570416,64023899339042,59821876591776,35507492708256];

macro_rules! index {
    ($input:tt, $n:literal) => {
        ($input[$n] as usize *10 + $input[$n+1] as usize)*10 + $input[$n+2] as usize - 5328
    }
}

macro_rules! ans {
    ($LUT:tt, $input:tt) => {
        $LUT[index!($input, 0)] +
        $LUT[index!($input, 5)] +
        $LUT[index!($input,10)] +
        $LUT[index!($input,15)] +
        $LUT[index!($input,20)]
    }
}

#[inline(always)]
#[aoc(day21, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    ans!(LUT1, input)
}
#[inline(always)]
#[aoc(day21, part2)]
pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    ans!(LUT2, input)
}
