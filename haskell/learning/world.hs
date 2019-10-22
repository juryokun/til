{-# LANGUAGE OverloadedStrings #-}
import qualified Data.Text                     as T

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
