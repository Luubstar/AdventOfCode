module Dia4 where 
 import Data.List.Split (splitOn)
 import Debug.Trace (trace)

 part1 :: [String] -> Int 
 part1 (cabeza:lista) = (obtenerValor cabeza) + (part1 lista) -- Se itera a través de las lineas para obtener su valor 
 part1 [] = 0 -- Si la lista es vacía, devuelve 0 (Caso base)
 
 obtenerValor :: String -> Int
 obtenerValor cadena = do
  let listas = splitOn "|" $ ((splitOn ":" cadena)!!1) -- Genera dos listas, la de los número y la de los números ganadores
      exp    = obtenerExponente (words $ tail $ init $ show (listas!!0)) (words $ init $ tail $ show (listas!!1)) -- Calculamos la cantidad de ganadores - 1
  if (exp >= 0) then 2^(exp) else 0 -- Devolvemos 2 ^ exp 

 obtenerExponente :: [String] -> [String] -> Int
 obtenerExponente numeros ganadores = (length $ filter id $ map (`elem` ganadores) numeros)  - 1 -- Crea una lista de booleanos siendo estos los ganadores, cuenta los trues, elimina 1

 -----------PARTE 2---------------------- 
 
 part2 :: [String] -> Int
 part2 lista = extraerValor $ extraerCopias (obtenerLista lista)

 extraerValor :: [(Int, Bool, Int)] -> Int
 extraerValor ((c, g, v):lista ) = c + (extraerValor lista)
 extraerValor [] = 0

 extraerCopias :: [(Int, Bool, Int)] -> [(Int, Bool, Int)]
 extraerCopias ((cantidad, ganadora, victorias):lista) 
  | ganadora =
     if (victorias > (length lista)) then add lista ((length lista) - victorias) cantidad else do
      let nueval =  add lista victorias cantidad
      [(cantidad, ganadora, victorias)] ++ extraerCopias nueval
  | otherwise = [(cantidad, ganadora, victorias)] ++ extraerCopias lista
 extraerCopias [] = []

 add :: [(Int, Bool, Int)] -> Int -> Int -> [(Int, Bool, Int)]
 add ((cantidad, ganadora, victorias): lista) i reps 
  | i == 0 = [(cantidad, ganadora, victorias)] ++ add lista 0 reps
  | otherwise = [(cantidad + reps, ganadora, victorias)] ++ (add lista (i-1)) reps
 add [] _ _ = []

 obtenerLista :: [String] -> [(Int, Bool, Int)]
 obtenerLista (cabeza:lista)
  | (obtenerValor' cabeza) > 0 = [(1, True, obtenerValor' cabeza)] ++ obtenerLista lista
  | otherwise = [(1, False, 0)] ++ obtenerLista lista
 obtenerLista [] = []

 obtenerValor' :: String -> Int
 obtenerValor' cadena = do
  let listas = splitOn "|" $ ((splitOn ":" cadena)!!1) -- Genera dos listas, la de los número y la de los números ganadores
      val    = obtenerExponente' (words $ tail $ init $ show (listas!!0)) (words $ init $ tail $ show (listas!!1)) -- Calculamos la cantidad de ganadores - 1
  val 

 obtenerExponente' :: [String] -> [String] -> Int
 obtenerExponente' numeros ganadores = (length $ filter id $ map (`elem` ganadores) numeros) -- Crea una lista de booleanos siendo estos los ganadores, cuenta los trues, elimina 1


 main = do 
   contents <- (readFile "Input4.txt")
   let lin = lines contents
   --putStrLn $ show $ part1 lin 
   putStrLn "--------"
   putStrLn $ show $ part2 lin 
