import           Data.List.Split
reverser :: IO ()
reverser = do
  input <- getContents
  let reversed = reverse input
  putStrLn reversed

main :: IO ()
-- main = do
--   userInput <- getContents
--   let numbers = toInts userInput
--   print (sum numbers)
main = do
  userInput <- getContents
  let ints        = toInts userInput
  let squaredList = map (^ 2) ints
  putStrLn (show (sum squaredList))

-- squareList :: String -> [Int]
-- squareList ls = map (read . lines) ls

sampleData = ['6', '2', '\n', '2', '1', '\n']

myLines = splitOn "\n"

toInts :: String -> [Int]
toInts = map read . lines
