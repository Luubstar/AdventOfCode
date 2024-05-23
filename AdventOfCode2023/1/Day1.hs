module Day1 where
 import Data.Char(isDigit, digitToInt)
 import Data.List(isPrefixOf)

 part1 :: [String] -> Int
 part1 (x:xs) = (toInt x) + (part1 xs)
 part1 [] = 0

 toInt :: String -> Int
 toInt x = read $ [head $ toCharArray x, last $ toCharArray x]
 
 toCharArray :: String -> [Char]
 toCharArray (x:xs)
  | isDigit x = (x) : (toCharArray xs)
  | otherwise = toCharArray xs
 toCharArray [] = [] 

 part2 :: [String] -> Int
 part2 (x:xs) =(toInt' x) + (part2 xs)
 part2 [] = 0

 toInt' :: String -> Int
 toInt' x = read $ [head $ toCharArray' x, last $ toCharArray' x]
 
 toCharArray' :: String -> [Char]
 toCharArray' (x:xs)
  | isDigit x = (x) : (toCharArray' xs)
  | b =  (spelledToChar (x:xs)) : (toCharArray' xs)
  | otherwise = toCharArray' xs
  where 
   l = (x:xs)
   b = isPrefixOf "one" l || isPrefixOf "two" l || isPrefixOf "three" l || isPrefixOf "four" l || isPrefixOf "five" l || isPrefixOf "six" l|| isPrefixOf "seven" l|| isPrefixOf "eight" l|| isPrefixOf "nine" l
 toCharArray' [] = [] 

 spelledToChar :: String -> Char
 spelledToChar l 
   | isPrefixOf "one" l = '1'
   | isPrefixOf "two" l = '2'
   | isPrefixOf "three" l = '3'
   | isPrefixOf "four" l = '4'
   | isPrefixOf "five" l = '5'
   | isPrefixOf "six" l = '6'
   | isPrefixOf "seven" l = '7'
   | isPrefixOf "eight" l = '8'
   | isPrefixOf "nine" l = '9'
   | otherwise = '_'


 main = do
  contents <- (readFile "Input1.txt")
  let lin = lines contents
  putStrLn $ show $ part1 lin
  putStrLn "--------"
  putStrLn $ show $ part2 lin

