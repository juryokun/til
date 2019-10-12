import           Data.List
import           Data.Semigroup
import           qualified Data.Map as Map

aPet :: [String]
aPet = ["cat", "dog"]

-- name :: (String, String)
-- name = ("Oscar", "mon")

halve :: Integer -> Integer
halve value = value `div` 2

anotherNumber :: Double
anotherNumber = read "6"

printDouble :: Int -> String
printDouble value = show (value * 2)

myHead :: [a] -> a
myHead (x : xs) = x

myTail :: [a] -> [a]
myTail []       = []
myTail (x : xs) = xs

myFoldl :: (a -> b -> a) -> a -> [b] -> a
myFoldl f init []       = init
myFoldl f init (x : xs) = myFoldl f newlInit xs where newlInit = f init x

-- patientInfo :: String -> String -> Int -> Int -> String
-- patientInfo fname lname age height = name ++ " " ++ ageHeight
--   where name = lname ++ ", " ++ fname
--         ageHeight = "(" ++ show age ++ ")" ++ "yrs. " ++ show height ++ "in.)"

type FirstName = String
type LastName = String
type Age = Int
type Height = Int

type PatientName = (String, String)
firstName :: PatientName -> String
firstName patient = fst patient
lastName :: PatientName -> String
lastName patient = snd patient

patientInfo :: PatientName -> Age -> Height -> String
patientInfo (fname, lname) age height = name ++ " " ++ ageHeight
 where
  name      = lname ++ ", " ++ fname
  ageHeight = "(" ++ show age ++ ")" ++ "yrs. " ++ show height ++ "in.)"

data Sex = Male | Female
sexInitial :: Sex -> Char
sexInitial Male   = 'M'
sexInitial Female = 'F'

data RhType = POS | Neg
data ABOType = A | B | AB | O
data BloodType = BloodType ABOType RhType

patient1BT :: BloodType
patient1BT = BloodType A POS

patient2BT :: BloodType
patient2BT = BloodType O Neg

patient3BT :: BloodType
patient3BT = BloodType AB POS

showRh :: RhType -> String
showRh POS = "+"
showRh Neg = "-"

showABO :: ABOType -> String
showABO A  = "A"
showABO B  = "B"
showABO AB = "AB"
showABO O  = "O"

showBloodType :: BloodType -> String
showBloodType (BloodType abo rh) = showABO abo ++ showRh rh

canDonateTo :: BloodType -> BloodType -> Bool
canDonateTo (BloodType O _) _                = True
canDonateTo _               (BloodType AB _) = True
canDonateTo (BloodType A _) (BloodType A  _) = True
canDonateTo (BloodType B _) (BloodType B  _) = True
canDonateTo _               _                = False

type MiddleName = String
data Name = Name FirstName LastName | NameWithMiddle FirstName MiddleName LastName

showName :: Name -> String
showName (Name f l            ) = f ++ " " ++ l
showName (NameWithMiddle f m l) = f ++ " " ++ m ++ " " ++ l

name1 = Name "Jerome" "Salinger"
name2 = NameWithMiddle "Jerome" "David" "Salinger"

data Patient = Patient {name :: Name
                , sex :: Sex
                , age :: Int
                , height :: Int
                , weight :: Int
                , bloodType :: BloodType
}

jackieSmith :: Patient
jackieSmith = Patient { name      = Name "Jackie" "Smith"
                      , age       = 43
                      , sex       = Female
                      , height    = 62
                      , weight    = 115
                      , bloodType = BloodType O Neg
                      }

donorFor :: Patient -> Patient -> Bool
donorFor p1 p2 = canDonateTo (bloodType p1) (bloodType p2)

showSex :: Sex -> String
showSex Male   = "Male"
showSex Female = "Female"

patientSummary :: Patient -> String
patientSummary patient =
  "Patient Name: "
    ++ showName (name patient)
    ++ "\n"
    ++ "Sex: "
    ++ showSex (sex patient)
    ++ "\n"
    ++ "Age: "
    ++ show (age patient)
    ++ "\n"
    ++ "Height: "
    ++ show (height patient)
    ++ "\n"
    ++ "Weight: "
    ++ show (weight patient)
    ++ "\n"
    ++ "Blood Type: "
    ++ showBloodType (bloodType patient)
    ++ "\n"

inc :: Int -> Int
inc x = x + 1

cycleSucc :: (Bounded a, Enum a, Eq a, Num a) => a -> a
cycleSucc n = if n == maxBound then minBound else succ n

myRep :: (Bounded a, Bounded b) => a -> b -> a
myRep x y = maxBound

data NewEngland = ME | VT | NH | MA | RI | CT
-- data SixSidedDie = S1 | S2 | S3 | S4 | S5 | S6 deriving (Eq, Ord)
-- data SixSidedDie = S1 | S2 | S3 | S4 | S5 | S6 deriving(Show)
data SixSidedDie = S1 | S2 | S3 | S4 | S5 | S6
instance Show SixSidedDie where
  show S1 = "one"
  show S2 = "two"
  show S3 = "three"
  show S4 = "four"
  show S5 = "five"
  show S6 = "six"

-- data TwoSidedDie = One | Two

-- show :: TwoSidedDie -> String
-- show One = "one"
-- show Two = "two"
-- instance Eq SixSidedDie where
--   (==) S6 S6 = True
--   (==) S5 S5 = True
--   (==) S4 S4 = True
--   (==) S3 S3 = True
--   (==) S2 S2 = True
--   (==) S1 S1 = True
--   (==) _  _  = False

data Test1 = AA | ZZ deriving (Eq, Ord)
data Test2 = ZZZ | AAA deriving (Eq, Ord)

instance Enum SixSidedDie where
  toEnum 0 = S1
  toEnum 1 = S2
  toEnum 2 = S3
  toEnum 3 = S4
  toEnum 4 = S5
  toEnum 5 = S6
  toEnum _ = error " no such value"

  fromEnum S1 = 0
  fromEnum S2 = 1
  fromEnum S3 = 2
  fromEnum S4 = 3
  fromEnum S5 = 4
  fromEnum S6 = 5

-- type NameA = (String, String)

-- names :: [Name]
-- names =
--   [("Emill", "Cioran"), ("Eugene", "Thacker"), ("Friedrich", "Nietzsche")]

-- instance Ord Name where
--   compare (f1, l1) (f2, l2) = compare (l1, f1) (l2, f2)

data NameA = NameA (String, String) deriving (Show, Eq)
instance Ord NameA where
  compare (NameA (f1, l1)) (NameA (f2, l2)) = compare (l1, f1) (l2, f2)

names :: [NameA]
names =
  [ NameA ("Emil", "Cioran")
  , NameA ("Eugene", "Thacker")
  , NameA ("Friedrich", "Nietzsche")
  ]

data FourLetterAlphabet = L1 | L2 | L3 | L4 deriving (Show, Enum, Bounded)
rotN :: (Bounded a, Enum a) => Int -> a -> a
rotN alphabetSize c = toEnum rotation
 where
  halfAlphabet = alphabetSize `div` 2
  offset       = fromEnum c + halfAlphabet
  rotation     = offset `mod` alphabetSize

largestCharNumber :: Int
largestCharNumber = fromEnum (maxBound :: Char)

rotChar :: Char -> Char
rotChar charToEncrypt = rotN sizeOfAlphabet charToEncrypt
  where sizeOfAlphabet = 1 + fromEnum (maxBound :: Char)

fourLetterMessage :: [FourLetterAlphabet]
fourLetterMessage = [L1, L3, L4, L1, L1, L2]

-- fourLetterEncoder :: [FourLetterAlphabet] -> [FourLetterAlphabet]
-- fourLetterEncoder vals = map rot4l vals
--  where
--   alphaSize = 1 + fromEnum (maxBound :: FourLetterAlphabet)
--   rot4l     = rotN alphaSize

data ThreeLetterAlphabet = Alpha | Beta | Kappa deriving (Show, Enum, Bounded)

threeLetterMessage :: [ThreeLetterAlphabet]
threeLetterMessage = [Alpha, Alpha, Beta, Alpha, Kappa]

-- threeLetterEncoder :: [ThreeLetterAlphabet] -> [ThreeLetterAlphabet]
-- threeLetterEncoder vals = map rot3l vals
--  where
--   alphaSize = 1 + fromEnum (maxBound :: ThreeLetterAlphabet)
--   rot3l     = rotN alphaSize

rotNdecoder :: (Bounded a, Enum a) => Int -> a -> a
rotNdecoder n c = toEnum rotation
 where
  halfN    = n `div` 2
  offset   = if even n then fromEnum c + halfN else 1 + fromEnum c + halfN
  rotation = offset `mod` n

-- threeLetterDecoder :: [ThreeLetterAlphabet] -> [ThreeLetterAlphabet]
-- threeLetterDecoder vals = map rot3ldecoder vals
--  where
--   alphaSize    = 1 + fromEnum (maxBound :: ThreeLetterAlphabet)
--   rot3ldecoder = rotNdecoder alphaSize

rotEncoder :: String -> String
rotEncoder text = map rotChar text
 where
  alphaSize = 1 + fromEnum (maxBound :: Char)
  rotChar   = rotN alphaSize

rotDecoder :: String -> String
rotDecoder text = map rotCharDecoder text
 where
  alphaSize      = 1 + fromEnum (maxBound :: Char)
  rotCharDecoder = rotNdecoder alphaSize

threeLetterEncoder :: [ThreeLetterAlphabet] -> [ThreeLetterAlphabet]
threeLetterEncoder vals = map rot3l vals
 where
  alphaSize = 1 + fromEnum (maxBound :: ThreeLetterAlphabet)
  rot3l     = rotN alphaSize

threeLetterDecoder :: [ThreeLetterAlphabet] -> [ThreeLetterAlphabet]
threeLetterDecoder vals = map rot3ldecoder vals
 where
  alphaSize    = 1 + fromEnum (maxBound :: ThreeLetterAlphabet)
  rot3ldecoder = rotNdecoder alphaSize

fourLetterEncoder :: [FourLetterAlphabet] -> [FourLetterAlphabet]
fourLetterEncoder vals = map rot4l vals
 where
  alphaSize = 1 + fromEnum (maxBound :: FourLetterAlphabet)
  rot4l     = rotN alphaSize

fourLetterDecoder :: [FourLetterAlphabet] -> [FourLetterAlphabet]
fourLetterDecoder vals = map rot4ldecoder vals
 where
  alphaSize    = 1 + fromEnum (maxBound :: FourLetterAlphabet)
  rot4ldecoder = rotNdecoder alphaSize

xorBool :: Bool -> Bool -> Bool
xorBool v1 v2 = (v1 || v2) && (not (v1 && v2))

xorPair :: (Bool, Bool) -> Bool
xorPair (v1, v2) = xorBool v1 v2

xor :: [Bool] -> [Bool] -> [Bool]
xor list1 list2 = map xorPair (zip list1 list2)

type Bits = [Bool]
intToBits' :: Int -> Bits
intToBits' 0 = [False]
intToBits' 1 = [True]
intToBits' n = if (remainder == 0)
  then False : intToBits' nextVal
  else True : intToBits' nextVal
 where
  remainder = n `mod` 2
  nextVal   = n `div` 2

maxBits :: Int
maxBits = length (intToBits' maxBound)

intToBits :: Int -> Bits
intToBits n = leadingFalses ++ reverseBits
 where
  reverseBits   = reverse (intToBits' n)
  missingBits   = maxBits - (length reverseBits)
  leadingFalses = take missingBits (cycle [False])

charToBits :: Char -> Bits
charToBits char = intToBits (fromEnum char)

bitsToInt :: Bits -> Int
bitsToInt bits = sum (map (\x -> 2 ^ (snd x)) trueLocations)
 where
  size          = length bits
  indices       = [size - 1, size - 2 .. 0]
  trueLocations = filter (\x -> fst x == True) (zip bits indices)

bitsToChar :: Bits -> Char
bitsToChar bits = toEnum (bitsToInt bits)

myPad :: String
myPad = "Shhhhhh"

myPlainText :: String
myPlainText = "Haskell"

applyOTP' :: String -> String -> [Bits]
applyOTP' pad plainText = map (\pair -> (fst pair) `xor` (snd pair))
                              (zip padBits plaintextBits)
 where
  padBits       = map charToBits pad
  plaintextBits = map charToBits plainText

applyOTP :: String -> String -> String
applyOTP pad plainText = map bitsToChar bitList
  where bitList = applyOTP' pad plainText

encoderDecoder :: String -> String
encoderDecoder = applyOTP myPad

class Cipher a where
  encode :: a -> String -> String
  decode :: a -> String -> String

data Rot = Rot

instance Cipher Rot where
  encode Rot text = rotEncoder text
  decode Rot text = rotDecoder text

data OneTimePad = OTP String

instance Cipher OneTimePad where
  encode (OTP pad) text = applyOTP pad text
  decode (OTP pad) text = applyOTP pad text

myOTP :: OneTimePad
myOTP = OTP (cycle [minBound .. maxBound])

-- puzzle
changeDesimal' :: Int -> Int -> [Int]
changeDesimal' 0 x = []
changeDesimal' n x = remainder : changeDesimal' nextVal x
 where
  remainder = n `mod` x
  nextVal   = n `div` x

changeDesimal :: Int -> Int -> [Int]
changeDesimal n x = reverse (changeDesimal' n x)

checkReversible :: [Int] -> Bool
checkReversible list = list == (reverse list)

puzzle1 x y z n = if xDesimal && yDesimal && zDesimal
  then n
  else puzzle1 x y z (n + 1)
 where
  xDesimal = checkReversible (changeDesimal n x)
  yDesimal = checkReversible (changeDesimal n y)
  zDesimal = checkReversible (changeDesimal n z)

myLast :: [a] -> a
myLast = head . reverse

myMin :: Ord a => [a] -> a
myMin = head . sort

myAll :: (a -> Bool) -> [a] -> Bool
myAll testFunc = (foldr (&&) True) . (map testFunc)

instance Semigroup Integer where
  (<>) x y = x + y

data Color = Red | Yellow | Blue | Green | Purple | Orange | Brown deriving (Show, Eq)

-- instance Semigroup Color where
--   (<>) Red    Blue   = Purple
--   (<>) Blue   Red    = Purple
--   (<>) Yellow Blue   = Green
--   (<>) Blue   Yellow = Green
--   (<>) Yellow Red    = Orange
--   (<>) Red    Yellow = Orange
--   (<>) a      b      = if a == b then a else Brown

instance Semigroup Color where
  (<>) Red    Blue   = Purple
  (<>) Blue   Red    = Purple
  (<>) Yellow Blue   = Green
  (<>) Blue   Yellow = Green
  (<>) Yellow Red    = Orange
  (<>) Red    Yellow = Orange
  (<>) a b | a == b    = a
           | all (`elem` [Red, Blue, Purple]) [a, b] = Purple
           | all (`elem` [Blue, Yellow, Green]) [a, b] = Green
           | all (`elem` [Red, Yellow, Orange]) [a, b] = Orange
           | otherwise = Brown

type Events = [String]
type Probs = [Double]
data PTable = PTable Events Probs

createPTable :: Events -> Probs -> PTable
createPTable events probs = PTable events normalizedProbs
 where
  totalProbs      = sum probs
  normalizedProbs = map (\x -> x / totalProbs) probs

showPair :: String -> Double -> String
showPair event prob = mconcat [event, "|", show prob, "\n"]

instance Show PTable where
  show (PTable events probs) = mconcat pairs
    where pairs = zipWith showPair events probs

cartCombine :: (a -> b -> c) -> [a] -> [b] -> [c]
cartCombine func l1 l2 = zipWith func newL1 cycledL2
 where
  nToAdd     = length l2
  repeatedL1 = map (take nToAdd . repeat) l1
  newL1      = mconcat repeatedL1
  cycledL2   = cycle l2

combineEvents :: Events -> Events -> Events
combineEvents e1 e2 = cartCombine combiner e1 e2
  where combiner = (\x y -> mconcat [x, "-", y])

combineProbs :: Probs -> Probs -> Probs
combineProbs p1 p2 = cartCombine (*) p1 p2

instance Semigroup PTable where
  (<>) ptable1        (PTable [] []) = ptable1
  (<>) (PTable [] []) ptable2        = ptable2
  (<>) (PTable e1 p1) (PTable e2 p2) = createPTable newEvents newProbs
   where
    newEvents = combineEvents e1 e2
    newProbs  = combineProbs p1 p2

instance Monoid PTable where
  mempty  = PTable [] []
  mappend = (<>)

coin :: PTable
coin = createPTable ["heads", "tails"] [0.5, 0.5]

spiner :: PTable
spiner = createPTable ["red", "blue", "green"] [0.1, 0.2, 0.7]

data Box a = Box a deriving Show

wrap :: a -> Box a
wrap x = Box x

unwrap :: Box a -> a
unwrap (Box x) = x

data Triple a = Triple a a a deriving Show
type Point3D = Triple Double

aPoint :: Point3D
aPoint = Triple 0.1 53.2 12.3

type FullName = Triple String

aPerson :: FullName
aPerson = Triple "Howard" "Phillips" "Lovercraft"

type Initials = Triple Char

initials :: Initials
initials = Triple 'H' 'P' 'L'

first :: Triple a -> a
first (Triple x _ _) = x

second :: Triple a -> a
second (Triple _ x _) = x

third :: Triple a -> a
third (Triple _ _ x) = x

toList :: Triple a -> [a]
toList (Triple x y z) = [x,y,z]

transform :: (a -> a) -> Triple a -> Triple a
transform f (Triple x y z) = Triple (f x) (f y) (f z)

data List a = Empty | Cons a (List a) deriving Show

builtinEx1 :: [Int]
builtinEx1 = 1:2:3:[]

ourListEx1 :: List Int
ourListEx1 = Cons 1 (Cons 2 (Cons 3 Empty))

builtinEx2 :: [Char]
builtinEx2 = 'c':'a':'t':[]

ourListEx2 :: List Char
ourListEx2 = Cons 'c' (Cons 'a' (Cons 't' Empty))

ourMap :: (a->b) -> List a -> List b
ourMap _ Empty = Empty
ourMap func (Cons a rest) = Cons (func a) (ourMap func rest)

itemCount1 :: (String, Int)
itemCount1 = ("Eraser", 25)

itemCount2 :: (String, Int)
itemCount2 = ("Pencils", 25)

itemCount3 :: (String, Int)
itemCount3 = ("Pens", 13)

itemInventory :: [(String, Int)]
itemInventory = [itemCount1, itemCount2, itemCount3]

data Organ = Heart | Brain | Kindney | Spleen deriving (Show, Eq)

organs :: [Organ]
organs = [Heart, Heart, Brain, Spleen, Spleen, Kindney]

ids :: [Int]
ids = [2,7,13,14,21,24]

-- pairs = [(2, Heart),(7,Heart),(13,Brain)..]
organPairs :: [(Int,Organ)]
organPairs = zip ids organs

organCatalog :: Map.Map Int Organ
organCatalog = Map.fromList organPairs

possibleDrawers :: [Int]
possibleDrawers = [1..50]

getDrawerContents :: [Int] -> Map.Map Int Organ -> [Maybe Organ]
getDrawerContents ids catalog = map getContents ids
  where getContents = \id -> Map.lookup id catalog

availableOrgans :: [Maybe Organ]
availableOrgans = getDrawerContents possibleDrawers organCatalog

countOrgan :: Organ -> [Maybe Organ] -> Int
countOrgan organ available = length (filter (\x -> x == Just organ) available)