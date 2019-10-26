{-# LANGUAGE OverloadedStrings #-}
import qualified Data.Text                     as T
import           Data.Semigroup

-- helloPerson :: String -> String
-- helloPerson name = "Hello" ++ " " ++ name ++ "!"

-- main :: IO ()
-- main = do
--   putStrLn "Hello! What's your name?"
--   name <- getLine
--   let statement = helloPerson name
--   putStrLn statement

firstWord :: String
firstWord = "pessimism"

secondWord :: T.Text
secondWord = T.pack firstWord

thirdWord :: String
thirdWord = T.unpack secondWord

myWord :: T.Text
myWord = "dog"

myNum1 :: Int
myNum1 = 5

sampleInput :: T.Text
sampleInput = "this\nis\ninput"

combinedTextMonoid :: T.Text
combinedTextMonoid = mconcat ["some", " ", "text"]

combinedTextSemigroup :: T.Text
combinedTextSemigroup = "some" <> " " <> "text"

myLines :: T.Text -> [T.Text]
myLines text = T.splitOn "\n" text

myUnlines :: [T.Text] -> T.Text
myUnlines textLines = T.intercalate "\n" textLines
