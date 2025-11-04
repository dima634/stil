const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Workspace_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Workspace {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Workspace>
}

impl ::protobuf::Message for Workspace {}

impl ::std::default::Default for Workspace {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Workspace {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Workspace` is `Sync` because it does not implement interior mutability.
//    Neither does `WorkspaceMut`.
unsafe impl Sync for Workspace {}

// SAFETY:
// - `Workspace` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Workspace {}

impl ::protobuf::Proxied for Workspace {
  type View<'msg> = WorkspaceView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Workspace {}

impl ::protobuf::MutProxied for Workspace {
  type Mut<'msg> = WorkspaceMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WorkspaceView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Workspace>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WorkspaceView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WorkspaceView<'msg> {
  type Message = Workspace;
}

impl ::std::fmt::Debug for WorkspaceView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WorkspaceView<'_> {
  fn default() -> WorkspaceView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Workspace>> for WorkspaceView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Workspace>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WorkspaceView<'msg> {

  pub fn to_owned(&self) -> Workspace {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional uint32
  pub fn id(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

}

// SAFETY:
// - `WorkspaceView` is `Sync` because it does not support mutation.
unsafe impl Sync for WorkspaceView<'_> {}

// SAFETY:
// - `WorkspaceView` is `Send` because while its alive a `WorkspaceMut` cannot.
// - `WorkspaceView` does not use thread-local data.
unsafe impl Send for WorkspaceView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for WorkspaceView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for WorkspaceView<'msg> {}

impl<'msg> ::protobuf::AsView for WorkspaceView<'msg> {
  type Proxied = Workspace;
  fn as_view(&self) -> ::protobuf::View<'msg, Workspace> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WorkspaceView<'msg> {
  fn into_view<'shorter>(self) -> WorkspaceView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Workspace> for WorkspaceView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Workspace {
    let mut dst = Workspace::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Workspace> for WorkspaceMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Workspace {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Workspace {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WorkspaceView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for WorkspaceMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WorkspaceMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Workspace>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WorkspaceMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WorkspaceMut<'msg> {
  type Message = Workspace;
}

impl ::std::fmt::Debug for WorkspaceMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Workspace>> for WorkspaceMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Workspace>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WorkspaceMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Workspace> {
    self.inner
  }

  pub fn to_owned(&self) -> Workspace {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional uint32
  pub fn id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}

// SAFETY:
// - `WorkspaceMut` does not perform any shared mutation.
unsafe impl Send for WorkspaceMut<'_> {}

// SAFETY:
// - `WorkspaceMut` does not perform any shared mutation.
unsafe impl Sync for WorkspaceMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for WorkspaceMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for WorkspaceMut<'msg> {}

impl<'msg> ::protobuf::AsView for WorkspaceMut<'msg> {
  type Proxied = Workspace;
  fn as_view(&self) -> ::protobuf::View<'_, Workspace> {
    WorkspaceView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WorkspaceMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Workspace>
  where
      'msg: 'shorter {
    WorkspaceView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for WorkspaceMut<'msg> {
  type MutProxied = Workspace;
  fn as_mut(&mut self) -> WorkspaceMut<'msg> {
    WorkspaceMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WorkspaceMut<'msg> {
  fn into_mut<'shorter>(self) -> WorkspaceMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Workspace {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Workspace> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WorkspaceView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WorkspaceMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional uint32
  pub fn id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

}  // impl Workspace

impl ::std::ops::Drop for Workspace {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Workspace {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Workspace {
  type Proxied = Self;
  fn as_view(&self) -> WorkspaceView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Workspace {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WorkspaceMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Workspace {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::Workspace_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$)P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::Workspace_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::Workspace_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Workspace {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Workspace {
  type Msg = Workspace;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Workspace> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Workspace {
  type Msg = Workspace;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Workspace> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WorkspaceMut<'_> {
  type Msg = Workspace;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Workspace> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WorkspaceMut<'_> {
  type Msg = Workspace;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Workspace> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WorkspaceView<'_> {
  type Msg = Workspace;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Workspace> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WorkspaceMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Client_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Client {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Client>
}

impl ::protobuf::Message for Client {}

impl ::std::default::Default for Client {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Client {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Client` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientMut`.
unsafe impl Sync for Client {}

// SAFETY:
// - `Client` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Client {}

impl ::protobuf::Proxied for Client {
  type View<'msg> = ClientView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Client {}

impl ::protobuf::MutProxied for Client {
  type Mut<'msg> = ClientMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Client>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientView<'msg> {
  type Message = Client;
}

impl ::std::fmt::Debug for ClientView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientView<'_> {
  fn default() -> ClientView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Client>> for ClientView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Client>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientView<'msg> {

  pub fn to_owned(&self) -> Client {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // address: optional string
  pub fn address(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // class: optional string
  pub fn class(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }

  // workspace: optional message Workspace
  pub fn has_workspace(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn workspace_opt(self) -> ::protobuf::Optional<super::WorkspaceView<'msg>> {
        ::protobuf::Optional::new(self.workspace(), self.has_workspace())
  }
  pub fn workspace(self) -> super::WorkspaceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WorkspaceView::default())
  }

}

// SAFETY:
// - `ClientView` is `Sync` because it does not support mutation.
unsafe impl Sync for ClientView<'_> {}

// SAFETY:
// - `ClientView` is `Send` because while its alive a `ClientMut` cannot.
// - `ClientView` does not use thread-local data.
unsafe impl Send for ClientView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ClientView<'msg> {}

impl<'msg> ::protobuf::AsView for ClientView<'msg> {
  type Proxied = Client;
  fn as_view(&self) -> ::protobuf::View<'msg, Client> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientView<'msg> {
  fn into_view<'shorter>(self) -> ClientView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Client> for ClientView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Client {
    let mut dst = Client::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Client> for ClientMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Client {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Client {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Client>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientMut<'msg> {
  type Message = Client;
}

impl ::std::fmt::Debug for ClientMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Client>> for ClientMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Client>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Client> {
    self.inner
  }

  pub fn to_owned(&self) -> Client {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

  // class: optional string
  pub fn class(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_class(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // workspace: optional message Workspace
  pub fn has_workspace(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_workspace(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn workspace_opt(&self) -> ::protobuf::Optional<super::WorkspaceView<'_>> {
        ::protobuf::Optional::new(self.workspace(), self.has_workspace())
  }
  pub fn workspace(&self) -> super::WorkspaceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WorkspaceView::default())
  }
  pub fn workspace_mut(&mut self) -> super::WorkspaceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_workspace(&mut self,
    val: impl ::protobuf::IntoProxied<super::Workspace>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `ClientMut` does not perform any shared mutation.
unsafe impl Send for ClientMut<'_> {}

// SAFETY:
// - `ClientMut` does not perform any shared mutation.
unsafe impl Sync for ClientMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ClientMut<'msg> {}

impl<'msg> ::protobuf::AsView for ClientMut<'msg> {
  type Proxied = Client;
  fn as_view(&self) -> ::protobuf::View<'_, Client> {
    ClientView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Client>
  where
      'msg: 'shorter {
    ClientView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ClientMut<'msg> {
  type MutProxied = Client;
  fn as_mut(&mut self) -> ClientMut<'msg> {
    ClientMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Client {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Client> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // address: optional string
  pub fn address(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_address(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        0,
        view,
      );
    }
  }

  // class: optional string
  pub fn class(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    // SAFETY: The runtime doesn't require ProtoStr to be UTF-8.
    unsafe { ::protobuf::ProtoStr::from_utf8_unchecked(str_view.as_ref()) }
  }
  pub fn set_class(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    let s = val.into_proxied(::protobuf::__internal::Private);
    let (view, arena) =
      s.into_inner(::protobuf::__internal::Private).into_raw_parts();

    let parent_arena = self.inner.arena();
    parent_arena.fuse(&arena);

    unsafe {
      self.inner.ptr_mut().set_base_field_string_at_index(
        1,
        view,
      );
    }
  }

  // workspace: optional message Workspace
  pub fn has_workspace(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_workspace(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn workspace_opt(&self) -> ::protobuf::Optional<super::WorkspaceView<'_>> {
        ::protobuf::Optional::new(self.workspace(), self.has_workspace())
  }
  pub fn workspace(&self) -> super::WorkspaceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WorkspaceView::default())
  }
  pub fn workspace_mut(&mut self) -> super::WorkspaceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_workspace(&mut self,
    val: impl ::protobuf::IntoProxied<super::Workspace>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl Client

impl ::std::ops::Drop for Client {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Client {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Client {
  type Proxied = Self;
  fn as_view(&self) -> ClientView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Client {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Client {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::Client_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::Client_msg_init.0, &[<super::Workspace as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::Client_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Client {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Client {
  type Msg = Client;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Client> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Client {
  type Msg = Client;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Client> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientMut<'_> {
  type Msg = Client;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Client> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientMut<'_> {
  type Msg = Client;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Client> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientView<'_> {
  type Msg = Client;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Client> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut Clients_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Clients {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Clients>
}

impl ::protobuf::Message for Clients {}

impl ::std::default::Default for Clients {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Clients {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Clients` is `Sync` because it does not implement interior mutability.
//    Neither does `ClientsMut`.
unsafe impl Sync for Clients {}

// SAFETY:
// - `Clients` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Clients {}

impl ::protobuf::Proxied for Clients {
  type View<'msg> = ClientsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Clients {}

impl ::protobuf::MutProxied for Clients {
  type Mut<'msg> = ClientsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClientsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Clients>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClientsView<'msg> {
  type Message = Clients;
}

impl ::std::fmt::Debug for ClientsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClientsView<'_> {
  fn default() -> ClientsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Clients>> for ClientsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Clients>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientsView<'msg> {

  pub fn to_owned(&self) -> Clients {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // clients: repeated message Client
  pub fn clients(self) -> ::protobuf::RepeatedView<'msg, super::Client> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Client>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `ClientsView` is `Sync` because it does not support mutation.
unsafe impl Sync for ClientsView<'_> {}

// SAFETY:
// - `ClientsView` is `Send` because while its alive a `ClientsMut` cannot.
// - `ClientsView` does not use thread-local data.
unsafe impl Send for ClientsView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientsView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for ClientsView<'msg> {}

impl<'msg> ::protobuf::AsView for ClientsView<'msg> {
  type Proxied = Clients;
  fn as_view(&self) -> ::protobuf::View<'msg, Clients> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientsView<'msg> {
  fn into_view<'shorter>(self) -> ClientsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Clients> for ClientsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Clients {
    let mut dst = Clients::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Clients> for ClientsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Clients {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Clients {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientsView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for ClientsMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClientsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Clients>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClientsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClientsMut<'msg> {
  type Message = Clients;
}

impl ::std::fmt::Debug for ClientsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Clients>> for ClientsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Clients>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClientsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Clients> {
    self.inner
  }

  pub fn to_owned(&self) -> Clients {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // clients: repeated message Client
  pub fn clients(&self) -> ::protobuf::RepeatedView<'_, super::Client> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Client>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn clients_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Client> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_clients(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Client>>) {
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.inner.arena().fuse(inner.arena());
    unsafe {
        <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
            .set_array_at_index(
                0,
                inner.raw());
    }
  }

}

// SAFETY:
// - `ClientsMut` does not perform any shared mutation.
unsafe impl Send for ClientsMut<'_> {}

// SAFETY:
// - `ClientsMut` does not perform any shared mutation.
unsafe impl Sync for ClientsMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for ClientsMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for ClientsMut<'msg> {}

impl<'msg> ::protobuf::AsView for ClientsMut<'msg> {
  type Proxied = Clients;
  fn as_view(&self) -> ::protobuf::View<'_, Clients> {
    ClientsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClientsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Clients>
  where
      'msg: 'shorter {
    ClientsView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for ClientsMut<'msg> {
  type MutProxied = Clients;
  fn as_mut(&mut self) -> ClientsMut<'msg> {
    ClientsMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClientsMut<'msg> {
  fn into_mut<'shorter>(self) -> ClientsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Clients {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Clients> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClientsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClientsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // clients: repeated message Client
  pub fn clients(&self) -> ::protobuf::RepeatedView<'_, super::Client> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::Client>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn clients_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::Client> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_clients(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::Client>>) {
    let val = src.into_proxied(::protobuf::__internal::Private);
    let inner = val.inner(::protobuf::__internal::Private);

    self.inner.arena().fuse(inner.arena());
    unsafe {
        <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
            .set_array_at_index(
                0,
                inner.raw());
    }
  }

}  // impl Clients

impl ::std::ops::Drop for Clients {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Clients {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Clients {
  type Proxied = Self;
  fn as_view(&self) -> ClientsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Clients {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClientsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Clients {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::Clients_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::Clients_msg_init.0, &[<super::Client as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::Clients_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Clients {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Clients {
  type Msg = Clients;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Clients> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Clients {
  type Msg = Clients;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Clients> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClientsMut<'_> {
  type Msg = Clients;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Clients> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientsMut<'_> {
  type Msg = Clients;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Clients> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClientsView<'_> {
  type Msg = Clients;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Clients> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClientsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



