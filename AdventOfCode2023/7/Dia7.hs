module Dia7 where 

-- ##############################################
-- ## Imports                                  ##
-- ##############################################
   import Data.List (sort, group, elemIndex, sortBy)
   import Data.Maybe (mapMaybe)
   import Debug.Trace (trace)
-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 
--
   valores :: [Char]
   valores = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
   
   data Naipe = Naipe String deriving Show  -- aka, es una String

   instance Eq Naipe where 
     (Naipe v) == (Naipe d) = (v) == (d) 

   instance Ord Naipe where 
    compare (Naipe valor1) (Naipe valor2) =  
     case compare v1 v2 of 
      EQ -> compareNaipes valor1 valor2
      resultado -> resultado
      where 
       v1 = getValue (Naipe valor1)  
       v2 = getValue (Naipe valor2) 
-- ##############################################
-- ## Función main                             ##
-- ##############################################

   main = do
     contents <- (readFile "Input7.txt")
     let lin = lines contents
         p1 = part1 $ map parseInput lin
     
     putStrLn $ show $ p1 
  
-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################
 
   part1 :: [(Naipe, Int)] -> Int 
   part1 n = 
     let lista = sort $ n
     in 
      getResult lista 1  

   getResult :: [(Naipe, Int)] -> Int -> Int
   getResult ((_,v):xs) length = (v * length) + (getResult xs (length+1)) 
   getResult [] _ = 0


-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################


-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

   parseInput :: String -> (Naipe, Int) 
   parseInput s =  ((Naipe ((words s)!!0)), read $ (words s)!!1)

   compareNaipes :: String -> String -> Ordering
   compareNaipes (c1:rest1) (c2:rest2) 
    | p < b = GT 
    | p > b = LT 
    | p == b = compareNaipes rest1 rest2
    where 
      p = elemIndex c1 valores 
      b = elemIndex c2 valores 

   countSame :: Naipe -> [Int]
   countSame (Naipe naipe) = map length $ group $ sort naipe

            


   getValue :: Naipe -> Int
   getValue naipe  
    | (head iguales) == 5 = 6
    | (head iguales) == 4 = 5
    | (head iguales) == 3 && (head $ drop 1 iguales) == 2 = 4
    | (head iguales) == 3 && (head $ drop 1 iguales) /= 2 = 3
    | (head iguales) == 2 && (head $ drop 1 iguales) == 2 = 2
    | (head iguales) == 2 && (head $ drop 1 iguales) /= 2 = 1
    | otherwise = 0
    where 
      iguales = ( reverse $ sort $ countSame naipe)

