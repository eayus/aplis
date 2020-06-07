#pragma once

#include <efi/types.h>
#include <efi/system-table.h>

char16_t digit_to_char(int n) {
	return n + L'0';
}

void print_char(efi_simple_text_output_protocol* ConOut, char16_t c) {
	char16_t str[2] = {c, '\0'};
	ConOut->OutputString(ConOut, str); 
}

void print_digit(efi_simple_text_output_protocol* ConOut, int d) {
	print_char(ConOut, digit_to_char(d));
}

void print_dec(efi_simple_text_output_protocol* ConOut, int n) {
	if (n < 10) {
		print_digit(ConOut, n);
	} else {
		print_dec(ConOut, n / 10);
		print_digit(ConOut, n % 10);
	}
}

void print(efi_simple_text_output_protocol* ConOut, char16_t* str) {
	ConOut->OutputString(ConOut, str);
}
