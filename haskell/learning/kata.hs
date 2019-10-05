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
