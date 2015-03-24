#!/usr/bin/python
# -*- coding: iso-8859-1 -*-

# run this as > $ python lex.py input.rs

import sys
from collections import OrderedDict

inFile = sys.argv[1]

reservadas = ["module","box","input","output","t_signal","p_signal","var","initially","up","activate","on_exception","emit"]
operadores = ["===>","--->","=",">","<",">=","<=","+","-","&","/"]
separadores = ["[","]",",",":","#","(",")",".","\'"]

def gera_tokens(inFile):
	arq = open("output.txt","wb")

	with open(inFile,'r') as f:
		palavra = []
		comentario = []
		reservada = []
		separador = []
		operador = []
		indicador = []
		digito = []
		lista = []
		tokens = []
		row = 1
		row_l = []
		col = 0
		last_col = 0
		
		while True:
			caracter = f.read(1)
			col += 1
			if caracter == "/":
				caracter = f.read(1)
				col += 1
				if caracter == "*":
					segue = True
					while segue:
						caracter = f.read(1)
						col += 1
						comentario.append(caracter)
						if caracter == "\n":
							row +=1
							last_col = col
							col = 1
						if caracter == "*":
							caracter = f.read(1)
							col+=1
							if caracter == "/":	
								col+=1
								segue = False
							else:
								comentario.append("*")
						if not caracter: 
							row -= 1
							print "Erro: comentário não fecha. Linha " + str(row) + " coluna " + str(col)
							exit()
				else:
					operador.append("/")
			else:
				if caracter != " " and caracter != "\n" and caracter != "\r":
					palavra.append(caracter)
				elif caracter == "\n":
					row += 1
					last_col = col
					col = 0
				else:
					lista.append(palavra)
					if palavra:
						erro = False
						for a in palavra:
							if a.isalpha() and palavra[0].isdigit():
								erro=True
								break
							if not palavra in separadores:
								if not a.isalpha() and not a.isdigit and not a == "_" and not a in operadores:
									erro = True
									break
						if erro:
							palavra = "".join(palavra)
							arq.write("possui erro na linha " + str(row) + " coluna " + str(col-len(palavra)) + "\n")
							arq.write(" > " + palavra)
							arq.close()
							print "possui erro na linha ",row, " coluna ",col-len(palavra)
							print palavra
							exit()
					palavra = []
				if not caracter: 
					col = last_col-1
					row -= 1
					break



		for word in lista:
			#print word
			for d in separadores:
				if d in word:
					tokens.append(d)
					separador.append(d)
					word = [w.replace(d, "|") for w in word]
			for d in operadores:
					tokens.append(d)
					operador.append(d)
					word = [w.replace(d, "|") for w in word]
			word = "".join(word)
			words = word.split("|")
			for w in words:
				if w in reservadas:
					tokens.append(w)
					reservada.append(w)
					words.remove(w)
				elif w in operadores:
					tokens.append(w)
					operador.append(w)
					words.remove(w)
				else:
					tokens.append(w)
					indicador.append(w)
					words.remove(w)
			for o in indicador:
				if o == "":
					indicador.remove(o)
				if o.isdigit():
					digito.append(o)
					indicador.remove(o)
			for t in tokens:
				if t == "":
					tokens.remove(t)
		for i in indicador:
			for w in i:
				if not w.isdigit() and not w.isalpha() and not w == "_":
					print "Erro: \"" + w + "\" não é reconhecido nesta linguagem"
					arq.write("Erro: \"" + w + "\" não é reconhecido nesta linguagem\n")
					arq.close()
					exit()
		

		tokens = list(OrderedDict.fromkeys(tokens))
		for t in tokens: 
			if t in reservada:
		#		print t + " - palavra reservada"
				arq.write(t + " - palavra reservada\n")
			elif t in indicador:
		#		print t + " - indicador"
				arq.write(t + " - indicador\n")
			elif t in separador:
		#		print t + " - separador"
				arq.write(t + " - separador\n")
			elif t in digito:
		#		print t + " - constante numérica"
				arq.write(t + " - constante numérica\n")
			elif t in operador:
		#		print t + " - operador"
				arq.write(t + " - operador\n")
			else:
				print t
				arq.write(t)

	arq.close()
	return tokens

tokens = gera_tokens(inFile)

def verifica_codigo(inFile):
	with open(inFile,'r') as f:
		palavra = []
		lista = []
		row = 1
		row_l = []
		col = 0
		last_col = 0

		while True:
			caracter = f.read(1)
			col += 1
			if caracter != " " and caracter != "\n" and caracter != "\r":
				palavra.append(caracter)
			elif caracter == "\n":
				row += 1
				last_col = col
				col = 0
			else:
				lista.append(palavra)
				palavra = []
			if not caracter: 
				col = last_col-1
				row -= 1
				break
		
		words = []
		for word in lista:
			for d in separadores:
				if d in word:
					word = [w.replace(d, "|") for w in word]
			for d in operadores:
				if d in word:
					word = [w.replace(d, "|") for w in word]
			word = "".join(word)
			words.append(word.split("|"))

verifica_codigo(inFile)