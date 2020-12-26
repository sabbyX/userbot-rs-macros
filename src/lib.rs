/* 
 * This file is part of the userbot-rs-macros (github.com/sabbyX/userbot-rs-macros).
 * Copyright (c) 2020 Sabby.
 * Copyright (c) Carapax authors (github.com/tg-rs/carapax)
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

//! Implementation of `dispatcher` aka function to designate command to a handler
//! Inspired from [Carapax](https://github.com/tg-rs/carapax) dispatcher implementation 
mod meta;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as meta::HandlerMeta);
    let args = if args.is_empty() {
        None
    } else {
        Some(parse_macro_input!(args as meta::CommandMeta))
    };
    TokenStream::from(build(input, args))
}

fn build(handler: meta::HandlerMeta, args: Option<meta::CommandMeta>) -> proc_macro2::TokenStream {
    let meta::HandlerMeta {
        ident,
        handler,
        ident_inner,
    } = handler;
    let mut inner_call = quote!(#ident_inner(message, client).await);
    match args {
        None => {}
        Some(command) => {
            let command = command.command;
            inner_call = quote!(
                if message.text().starts_with(#command) {
                    #inner_call
                } else {
                    Ok(())
                }
            )
        }
    }
    quote!(
        #handler
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct #ident;
        #[::async_trait::async_trait]
        impl super::core::handler::Handler for #ident {
            async fn handle(&self, message: Message, client: ClientHandle) -> ::anyhow::Result<()> {
                #inner_call
            }
        }
    )
}
