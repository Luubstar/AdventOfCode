
-- ##############################################
-- ## Imports                                  ##
-- ##############################################

import Data.List.Split (splitOn)
import Data.List (elem, notElem)
import Debug.Trace (trace)
import Data.List (intercalate)
import qualified Data.Array as A

-- ##############################################
-- ## Definiciones de datos, tipos y clases    ##
-- ############################################## 

type Grupo = (String, [Int]) 

-- ##############################################
-- ## Función main                             ##
-- ##############################################

main = do
 contents <- (readFile "Input12.txt")
 let lin = lines contents
 putStrLn $ show $ sum $ map part1 lin
 putStrLn "--------"
 putStrLn $ show $ sum $ map part2 lin 

-- ##############################################
-- ## Funciones de la 1º parte                 ##
-- ##############################################

part1 :: String -> Int  
part1 a = 
 let b = parseGrupos a
 in solve (fst b) (snd b) 0 "" 

solve :: String -> [Int] -> Int -> String -> Int  
solve entrada cuenta hashes req  
 | req == "#" && (lentrada == 0 || hentrada == '.') = 0 --si necesitamos un # y no hay espacio, devolvemos 0 
 | req == "." && almohadillas > 0 = 0 -- su necesitamos un punto pero empieza por # devolvemos un 0
 | (lcuenta) == 0 && not(hs) = 1  -- si no hay números que buscar y no falta nada, devolvemos 1
 | (lcuenta) == 0 && hs = 0 -- si todavía queda #, devolvemos 0
 | almohadillas > 0 = 
   if (lcuenta) == 0 then 0 else 
    if (hashes + almohadillas == head cuenta) 
      then solve  (drop almohadillas entrada) (tail cuenta) 0 "." 
      else 
        if (hashes + almohadillas < head cuenta) 
          then solve  (drop almohadillas entrada)  (cuenta) (hashes + almohadillas) "#"
          else 0

 | lentrada == 0 = 0 -- si no queda entrada, 0 
 | hentrada == '.' = solve (tail entrada) cuenta 0 "" -- si el primero de la entrada es ., pasamos
 | hentrada == '?' =  -- si es ?, devolvemos la suma causada al asumir que empieza por # y por .
   let a = solve ('#':tail entrada) cuenta hashes req
       b = solve ('.':tail entrada) cuenta hashes req 
   in a+b
  where 
   hs = elem '#' entrada
   almohadillas = getalmohadillas entrada
   lentrada = length entrada 
   hentrada = head entrada 
   lcuenta = length cuenta 

getalmohadillas :: String -> Int 
getalmohadillas (x:xs) 
 | x == '#' = 1 + getalmohadillas xs
 | otherwise = 0
getalmohadillas [] = 0
-- ##############################################
-- ## Funciones de la 2º parte                 ##
-- ##############################################

part2 :: String -> Int  
part2 a = 
 let b = parseGrupos' a
 in solve2 (fst b) (snd b) 0 ""  

-- ##############################################
-- ## Funciones generales                      ##
-- ##############################################

-- Devuelve una lista ("...", [1,2,3]) dado el input
parseGrupos :: String -> Grupo
parseGrupos x = 
 let split = splitOn " " x
     numeros = splitOn "," (split!!1)
 in (split!!0, map read numeros :: [Int])

parseGrupos' :: String -> Grupo
parseGrupos' x = 
 let split = splitOn " " x
     numeros = splitOn "," (split!!1)
 in (intercalate "?" $ replicate 5 (split!!0), concat $ replicate 5 (map read numeros :: [Int]))
