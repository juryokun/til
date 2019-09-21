import           Data.List
import           Data.Char

-- main = do
--     print "Who is the email for?"
--     recipient <- getLine
--     print "What is the Title?"
--     title <- getLine
--     print "Who is the Author?"
--     author <- getLine
--     print (createEmail recipient title author)
-- toPart recipient = "Dear" ++ recipient ++ ",\n"
-- bodyPart bookTitle = "Thanks for buying " ++ bookTitle ++ ".\n"
-- fromPart author = "Thanks,\n" ++ author
-- createEmail recipient bookTitle author = toPart recipient ++
--                         bodyPart bookTitle ++
--                         fromPart author
calcChange owed given = if change > 0 then change else 0
  where change = given - owed

-- sumSquareOrSquareSum x y = if sumSquare > squareSum
--                            then sumSquare
--                            else squareSum
--   where
--     sumSquare = x ^ 2 + y ^ 2
--     squareSum = (x + y) ^ 2
-- sumSquareOrSquareSum x y = (\sumSquare squareSum -> if sumSquare > squareSum
--                                                     then sumSquare
--                                                     else squareSum)
--   (x ^ 2 + y ^ 2)
--   ((x + y) ^ 2)
sumSquareOrSquareSum x y =
  let sumSquare = (x ^ 2 + y ^ 2)
      squareSum = ((x + y) ^ 2)
  in  if sumSquare > squareSum then sumSquare else squareSum

doubleDouble x = (\double -> double ^ 2) (x ^ 2)

makeChange given owed =
  (\change -> if change > 0 then change else 0) (given - owed)

counter x = let y = x + 1 in y
-- in let x = x + 1
-- counter x = (\x -> x + 1) ((\x -> x + 1) ((\x -> x) x))

ifEven myFunc x = if even x then myFunc x else 0

inc n = n + 1
double n = n * 2
square n = n ^ 2

ifEvenInc n = ifEven inc n
ifEvenDouble n = ifEven double n
ifEvenSquare n = ifEven square n

names =
  [ ("lan"    , "Curtis")
  , ("Bernard", "Sumner")
  , ("Aron"   , "Sumner")
  , ("Peter"  , "Hook")
  , ("Stephen", "Morris")
  ]

compareString string1 string2 =
  if string1 > string2 then GT else if string1 < string2 then LT else EQ

-- compareFirstName name1 name2 = compareString firstName1 firstName2
--  where
--   firstName1 = fst name1
--   firstName2 = fst name2

-- compareLastName name1 name2 = compareString lastName1 lastName2
--  where
--   lastName1 = snd name1
--   lastName2 = snd name2

compareNameBy fnc name1 name2 = compareString relName1 relName2
 where
  relName1 = fnc name1
  relName2 = fnc name2

compareName name1 name2 = if relCompareLastName == EQ
  then relCompareFirstName
  else relCompareLastName
 where
  relCompareLastName  = compareNameBy snd name1 name2
  relCompareFirstName = compareNameBy fst name1 name2

ifEven2 f x = if even x then f x else x

genIfXEven x = (\f -> ifEven2 f x)

-- RecieveURL
getRequestUrl host apikey resource id =
  host ++ "/" ++ resource ++ "/" ++ id ++ "?token=" ++ apikey

genHostRequestBuilder host =
  (\apikey resource id -> getRequestUrl host apikey resource id)

exampleUrlBuilder = genHostRequestBuilder "http://example.com"

genApiRequestBuilder hostBuilder apiKey =
  (\resource id -> hostBuilder apiKey resource id)

myExampleUrlBuilder = genApiRequestBuilder exampleUrlBuilder "1337hAsk3ll"

exampleUrlBuilder2 = getRequestUrl "http://example.com"
myExampleUrlBuilder2 = exampleUrlBuilder2 "1337hAsk3ll"

add4 a b c d = a + b + c + d
addXto3 x = (\b c d -> add4 x b c d)

subtract2 = flip (-) 2

ifEvenInc2 = ifEven inc
ifEvenDouble2 = ifEven double
ifEvenSquare2 = ifEven square

flipBinaryArgs binaryFunction = (\x y -> binaryFunction y x)
binaryPartialApplication f x = (\y -> f x y)

assignToGroups n aList = zip groups aList where groups = cycle [1 .. n]

repeatString t = cycle [t]

subseq start end myList = take diffence (drop start myList)
  where diffence = end - start

ifFirstHalf t myList = if elem t firstHalf then True else False
 where
  halfPoint = (length myList) `div` 2
  firstHalf = take halfPoint myList

myGCD a b = if remainder == 0 then b else myGCD b remainder
  where remainder = a `mod` b

sayAmount n = case n of
  1 -> "one"
  2 -> "two"
  n -> "a brunch"

-- myTail (_ : xs) = xs
-- myTail2 (x : xs) = xs
-- myHead (x : xs) = x
-- myHead2 (x : _) = x

myTail []       = []
myTail (_ : xs) = xs

sayAmount2 1 = "one"
sayAmount2 2 = "two"

sayAmount3 4 = False
sayAmount3 x = (mod x 2) == 0

myGCD2 a 0 = a
myGCD2 a b = myGCD2 b (mod a b)

myTake _ []       = []
myTake 0 _        = []
myTake n (x : xs) = x : rest where rest = myTake (n - 1) xs

finiteCycle (x : xs) = x : xs ++ finiteCycle (x : xs)
myCycle (x : xs) = x : myCycle (xs ++ [x])

ackermann 0 n = n + 1
ackermann m 0 = ackermann (m - 1) 1
ackermann m n = ackermann (m - 1) (ackermann m (n - 1))

collatz 1 = 1
collatz n = if even n then 1 + collatz (div n 2) else 1 + collatz (n * 3 + 1)

myReverse []       = []
myReverse (x : xs) = myReverse xs ++ [x]

fib 0 = 0
fib 1 = 1
fib n = fib (n - 1) + fib (n - 2)

fastFib _ _ 0 = 0
fastFib _ _ 1 = 1
fastFib _ _ 2 = 1
fastFib x y 3 = x + y
fastFib x y c = fastFib y (x + y) (c - 1)
fib2 n = fastFib 1 1 n

myFoldr f init []       = init
myFoldr f init (x : xs) = f x rightResult
  where rightResult = myFoldr f init xs

myElem val myList = (length filteredList) /= 0
  where filteredList = filter (== val) myList

isPalidrome string = changedStringByTrim == reverse changedStringByTrim
  where changedStringByTrim = map toLower (filter (/= ' ') string)

isPalidromeZ text = processdText == reverse processdText
 where
  noSpaces     = filter (/= ' ') text
  processdText = map toLower noSpaces

harmonic 1 = 1
harmonic n = 1 / n + harmonic (n - 1)

harmonicZ n = sum (take n seriesValues)
 where
  seriesPairs  = zip (cycle [1.0]) [1.0, 2.0 ..]
  seriesValues = map (\pair -> (fst pair) / (snd pair)) seriesPairs
