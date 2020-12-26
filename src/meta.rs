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
 
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Ident, ItemFn, Result as SynResult, LitStr, Token,
};

pub(crate) struct HandlerMeta {
    pub(super) ident_inner: Ident,
    pub(super) handler: ItemFn,
    pub(super) ident: Ident,
}

impl Parse for HandlerMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut handler = input.parse::<ItemFn>()?;
        if handler.sig.asyncness.is_none() {
            return Err(input.error("function must be async"));
        }
        let ident = handler.sig.ident.clone();
        let ident_inner = Ident::new(&format!("__userbot_{}", ident), Span::call_site());
        handler.sig.ident = ident_inner.clone();
        Ok(HandlerMeta {
            handler,
            ident_inner,
            ident
        })
    }
}

pub struct CommandMeta {
    pub(super) command: LitStr,
}

impl Parse for CommandMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let command = input.parse::<Ident>()?;
        let command = match command.to_string().as_ref() {
            "command" => {
                input.parse::<Token![=]>()?;
                let name = input.parse::<LitStr>()?;
                // TODO: Ability to set command prefix
                if !name.value().starts_with('*') {
                    return Err(input.error("Commands should startswith *"));
                }
                name
            }
            arg => {
                return Err(input.error(format!("Unexpected arg {}", arg)));
            }
        };
        Ok(CommandMeta {
            command,
        })
    }
}
