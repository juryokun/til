import           System.Environment
import           Control.Monad

main :: IO ()
-- main = do
--   args <- getArgs
--   mapM_ putStrLn args

-- main = do
--   args <- getArgs
--   let linesToRead = if length args > 0 then read (head args) else 0 :: Int
--   print linesToRead

-- main = do
--   args <- getArgs
--   let linesToRead = if length args > 0 then read (head args) else 0 :: Int
--   numbers <- replicateM linesToRead getLine
--   print "sum goes here"

main = do
  args <- getArgs
  let linesToRead = if length args > 0 then read (head args) else 0 :: Int
  numbers <- replicateM linesToRead getLine
  let ints = map read numbers :: [Int]
  print (sum ints)



exampleMain :: IO ()
exampleMain = do
  vals <- mapM (\_ -> getLine) [0 .. 2]
  mapM_ putStrLn vals
