aPet :: [String]
aPet = ["cat","dog"]

-- name :: (String, String)
-- name = ("Oscar", "mon")

halve :: Integer -> Integer
halve value = value `div` 2

anotherNumber :: Double
anotherNumber = read "6"

printDouble :: Int -> String
printDouble value = show (value * 2)

myHead :: [a] -> a
myHead (x:xs) = x

myTail :: [a] -> [a]
myTail [] = []
myTail (x:xs) = xs

myFoldl :: (a -> b -> a) -> a -> [b] -> a
myFoldl f init [] = init
myFoldl f init (x:xs) = myFoldl f newlInit xs
  where newlInit = f init x

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
  where name = lname ++ ", " ++ fname
        ageHeight = "(" ++ show age ++ ")" ++ "yrs. " ++ show height ++ "in.)"

data Sex = Male | Female
sexInitial :: Sex -> Char
sexInitial Male = 'M'
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
showABO A = "A"
showABO B = "B"
showABO AB = "AB"
showABO O = "O"

showBloodType :: BloodType -> String
showBloodType (BloodType abo rh) = showABO abo ++ showRh rh

canDonateTo :: BloodType -> BloodType -> Bool
canDonateTo (BloodType O _) _ = True
canDonateTo _ (BloodType AB _) = True
canDonateTo (BloodType A _) (BloodType A _) = True
canDonateTo (BloodType B _) (BloodType B _) = True
canDonateTo _ _ = False

type MiddleName = String
data Name = Name FirstName LastName | NameWithMiddle FirstName MiddleName LastName

showName :: Name -> String
showName (Name f l) = f ++ " " ++ l
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
jackieSmith = Patient {name = Name "Jackie" "Smith"
              , age = 43
              , sex = Female
              , height = 62
              , weight = 115
              , bloodType = BloodType O Neg
}