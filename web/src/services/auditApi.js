import request from "./request"

const prefix = "/0/api/v1/admin-rs"

function getAuditList({ content, status, categoryId, current, size }) {
    const params = {
        content,
        status,
        categoryId,
        current,
        size,
    }
    return request.get(`${prefix}/audit/list`, {
        params
    })
}

// 通知列表
function notificationList({ viewFlag, size, current }) {
    const url = `${prefix}/message/list`
    return request.get(url, {
        params: {
            viewFlag,
            size,
            current
        }
    })
}

// 改变为已读(read) 或者 删除(delete)
function changeStatusOrDetele({ id, viewFlag }) {
    const url = `${prefix}/message/update`;
    return request.post(url, {
        id,
        viewFlag,
    })
}

function getAuditDetail(id) {
    return request.get(`${prefix}/audit/get-detail`, {
        params: {
            id
        }
    })
}

function updateAudit({
    id,
    status,
    opinion
}) {
    return request.post(`${prefix}/audit/update`, {
        id,
        status,
        opinion
    })
}

// 判断是否还有未读信息
function isThreeAnyUnread({}){
    const url = `${prefix}/message/status`;
    return request.get(url, {params:{
        role: 10
    }})
    }

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

// 新通知列表
function notificationList2({viewFlag, size, current}){
    const url = `${prefix}/notification/message`
    return request.get(url, {params: {
      readFlag: viewFlag,
      pageSize: size,
      pageIndex: current
    }})
  }
  // 改变为已读 或者是全部已读
  function changeStatus({id, viewFlag}){
    const url = `${prefix}/notification/message`;
    return request.put(url,{ 
      id,
      viewFlag,
    })
  }
  // 删除
  function deleteNotification({id}){
    const url = `${prefix}/notification/message`;
    return request.delete(url, {data: {id}})
  }
  // 是否有未读
  function isUnRead(){
    const url = `${prefix}/notification/status`;
    return request.get(url)
  }

const auditApi = {
    getAuditList,
    getAuditDetail,
    updateAudit,
    notificationList,
    changeStatusOrDetele,
    isThreeAnyUnread,
    // ---
    notificationList2,
    changeStatus,
    deleteNotification,
    isUnRead
}

export default auditApi
