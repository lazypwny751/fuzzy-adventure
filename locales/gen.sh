#!/bin/bash

# Auto generator for locale rust file.
export OPT="help" SUFFIX=".sh" LOCALES="${PWD}" OUT="transcription.rs"

while (( "${#}" > 0 ))
do
	case "${1,,}" in
		("build"|"--build")
			export OPT="build"
			shift
		;;
		("--suffix")
			shift
			if [[ -n "${1}" ]]
			then
				export SUFFIX="${1}"
				shift
			fi
		;;
		("--locales")
			shift
			if [[ -n "${1}" ]]
			then
				export LOCALES="${1}"
				shift
			fi
		;;
		("--out")
			shift
			if [[ -n "${1}" ]]
			then
				export OUT="${1}"
				shift
			fi
		;;
		(*)
			shift
		;;
	esac
done

case "${OPT,,}" in
	("build")
		if [[ -d "${LOCALES}" ]]
		then
			for f in "${LOCALES}/"*"${SUFFIX}"
			do
				if [[ "${f##*/}" != "${0}" ]]
				then
					echo "${f}"
				fi
			done
		else
			echo -e "${0##*/}: error: the LOCALES direcotry \"${LOCALES##*/}\" doesn't exists."
			exit 1
		fi
	;;
	(*)
		echo "Help text."
	;;
esac
