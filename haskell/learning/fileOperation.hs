import           System.IO

main :: IO ()
-- main = do
--   helloFile <- openFile "hello.txt" ReadMode
--   firstLine <- hGetLine helloFile
--   putStrLn firstLine
--   secondLine  <- hGetLine helloFile
--   goodbyeFile <- openFile "goodbye.txt" AppendMode
--   hPutStrLn goodbyeFile secondLine
--   hClose helloFile
--   hClose goodbyeFile
--   putStrLn "done!"

main = do
  helloFile <- openFile "goodbye.txt" ReadMode
  hasLine   <- hIsEOF helloFile
  firstLine <- if not hasLine then hGetLine helloFile else return "empty"
  putStrLn firstLine
  putStrLn "done!"
