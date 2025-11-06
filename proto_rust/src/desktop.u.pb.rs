const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.33.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut protos__Desktop_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Desktop {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Desktop>
}

impl ::protobuf::Message for Desktop {}

impl ::std::default::Default for Desktop {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Desktop {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Desktop` is `Sync` because it does not implement interior mutability.
//    Neither does `DesktopMut`.
unsafe impl Sync for Desktop {}

// SAFETY:
// - `Desktop` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl Send for Desktop {}

impl ::protobuf::Proxied for Desktop {
  type View<'msg> = DesktopView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Desktop {}

impl ::protobuf::MutProxied for Desktop {
  type Mut<'msg> = DesktopMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DesktopView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Desktop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DesktopView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DesktopView<'msg> {
  type Message = Desktop;
}

impl ::std::fmt::Debug for DesktopView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DesktopView<'_> {
  fn default() -> DesktopView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Desktop>> for DesktopView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Desktop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DesktopView<'msg> {

  pub fn to_owned(&self) -> Desktop {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // active_window: optional message protos.Client
  pub fn has_active_window(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn active_window_opt(self) -> ::protobuf::Optional<super::ClientView<'msg>> {
        ::protobuf::Optional::new(self.active_window(), self.has_active_window())
  }
  pub fn active_window(self) -> super::ClientView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientView::default())
  }

  // clients: optional message protos.Clients
  pub fn has_clients(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clients_opt(self) -> ::protobuf::Optional<super::ClientsView<'msg>> {
        ::protobuf::Optional::new(self.clients(), self.has_clients())
  }
  pub fn clients(self) -> super::ClientsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientsView::default())
  }

}

// SAFETY:
// - `DesktopView` is `Sync` because it does not support mutation.
unsafe impl Sync for DesktopView<'_> {}

// SAFETY:
// - `DesktopView` is `Send` because while its alive a `DesktopMut` cannot.
// - `DesktopView` does not use thread-local data.
unsafe impl Send for DesktopView<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DesktopView<'msg> {}
impl<'msg> ::protobuf::ViewProxy<'msg> for DesktopView<'msg> {}

impl<'msg> ::protobuf::AsView for DesktopView<'msg> {
  type Proxied = Desktop;
  fn as_view(&self) -> ::protobuf::View<'msg, Desktop> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DesktopView<'msg> {
  fn into_view<'shorter>(self) -> DesktopView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Desktop> for DesktopView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Desktop {
    let mut dst = Desktop::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Desktop> for DesktopMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Desktop {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::runtime::EntityType for Desktop {
    type Tag = ::protobuf::__internal::runtime::MessageTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DesktopView<'msg> {
    type Tag = ::protobuf::__internal::runtime::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::runtime::EntityType for DesktopMut<'msg> {
    type Tag = ::protobuf::__internal::runtime::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DesktopMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Desktop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DesktopMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DesktopMut<'msg> {
  type Message = Desktop;
}

impl ::std::fmt::Debug for DesktopMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Desktop>> for DesktopMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Desktop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DesktopMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Desktop> {
    self.inner
  }

  pub fn to_owned(&self) -> Desktop {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // active_window: optional message protos.Client
  pub fn has_active_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_active_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn active_window_opt(&self) -> ::protobuf::Optional<super::ClientView<'_>> {
        ::protobuf::Optional::new(self.active_window(), self.has_active_window())
  }
  pub fn active_window(&self) -> super::ClientView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientView::default())
  }
  pub fn active_window_mut(&mut self) -> super::ClientMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_active_window(&mut self,
    val: impl ::protobuf::IntoProxied<super::Client>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // clients: optional message protos.Clients
  pub fn has_clients(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_clients(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn clients_opt(&self) -> ::protobuf::Optional<super::ClientsView<'_>> {
        ::protobuf::Optional::new(self.clients(), self.has_clients())
  }
  pub fn clients(&self) -> super::ClientsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientsView::default())
  }
  pub fn clients_mut(&mut self) -> super::ClientsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clients(&mut self,
    val: impl ::protobuf::IntoProxied<super::Clients>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `DesktopMut` does not perform any shared mutation.
unsafe impl Send for DesktopMut<'_> {}

// SAFETY:
// - `DesktopMut` does not perform any shared mutation.
unsafe impl Sync for DesktopMut<'_> {}

impl<'msg> ::protobuf::Proxy<'msg> for DesktopMut<'msg> {}
impl<'msg> ::protobuf::MutProxy<'msg> for DesktopMut<'msg> {}

impl<'msg> ::protobuf::AsView for DesktopMut<'msg> {
  type Proxied = Desktop;
  fn as_view(&self) -> ::protobuf::View<'_, Desktop> {
    DesktopView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DesktopMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Desktop>
  where
      'msg: 'shorter {
    DesktopView {
      inner: ::protobuf::__internal::runtime::MessageViewInner::view_of_mut(self.inner.clone())
    }
  }
}

impl<'msg> ::protobuf::AsMut for DesktopMut<'msg> {
  type MutProxied = Desktop;
  fn as_mut(&mut self) -> DesktopMut<'msg> {
    DesktopMut { inner: self.inner }
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DesktopMut<'msg> {
  fn into_mut<'shorter>(self) -> DesktopMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Desktop {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Desktop> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DesktopView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DesktopMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // active_window: optional message protos.Client
  pub fn has_active_window(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_active_window(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn active_window_opt(&self) -> ::protobuf::Optional<super::ClientView<'_>> {
        ::protobuf::Optional::new(self.active_window(), self.has_active_window())
  }
  pub fn active_window(&self) -> super::ClientView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientView::default())
  }
  pub fn active_window_mut(&mut self) -> super::ClientMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_active_window(&mut self,
    val: impl ::protobuf::IntoProxied<super::Client>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // clients: optional message protos.Clients
  pub fn has_clients(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_clients(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn clients_opt(&self) -> ::protobuf::Optional<super::ClientsView<'_>> {
        ::protobuf::Optional::new(self.clients(), self.has_clients())
  }
  pub fn clients(&self) -> super::ClientsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClientsView::default())
  }
  pub fn clients_mut(&mut self) -> super::ClientsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_clients(&mut self,
    val: impl ::protobuf::IntoProxied<super::Clients>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl Desktop

impl ::std::ops::Drop for Desktop {
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Desktop {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Desktop {
  type Proxied = Self;
  fn as_view(&self) -> DesktopView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Desktop {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DesktopMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Desktop {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::protos__Desktop_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::protos__Desktop_msg_init.0, &[<super::Client as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Clients as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::protos__Desktop_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Desktop {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Desktop {
  type Msg = Desktop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Desktop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Desktop {
  type Msg = Desktop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Desktop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DesktopMut<'_> {
  type Msg = Desktop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Desktop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DesktopMut<'_> {
  type Msg = Desktop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Desktop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DesktopView<'_> {
  type Msg = Desktop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Desktop> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DesktopMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



