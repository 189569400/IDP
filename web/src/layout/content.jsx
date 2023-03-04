import React, {Fragment, useEffect, useImperativeHandle, useState} from 'react'
import RightSiderShow from "../components/rightSiderLine/RightSiderShow"
import RightSideLine from "../components/rightSiderLine"
import RouterConfig from "../router/router"
import WorkspaceLeft from "../components/workspace/WorkspaceLeft"
import { useLocation} from "react-router"
import RenderVersionPanel from "../components/notebook/version/VersionPanel"
import DecisionClassification from '../components/notebook/operator/DecisionClassification'
import {selectActivePath} from "@/store/features/filesTabSlice"
import {useSelector} from "react-redux"
import {isTraveler} from "@/store/cookie"



export const contentContext = React.createContext()

function Content(props,ref) {


  const location = useLocation()
  const  activeTabPath = useSelector(selectActivePath)

  const [rightLineSelectKey, setRightLineSelectKey] = useState("")
  const [oldRightLineSelectKey, setOldRightLineSelectKey] = useState("")
  const [showSaveVersion, setShowSaveVersion] = useState(false)
  const [showVersionDrawer, setShowVersionDrawer] = useState(false)

  const setShowVersionDrawerWithOther = (showVersionDrawer) => {
    setShowVersionDrawer(showVersionDrawer)
    if (!showVersionDrawer) {
      setRightLineSelectKey(oldRightLineSelectKey)
    }
  }

  const setRightLineSelectKeyWithOther = (key, oldKey) => {
    setRightLineSelectKey(key)
    setOldRightLineSelectKey(oldKey)
  }

  const showRightLine = () => {
    const pathname = location.pathname
    const suffix = activeTabPath.slice(activeTabPath.lastIndexOf(".") + 1)
    return pathname.endsWith("/workspace") && (suffix === "ipynb" || suffix === "idpnb")
  }

  const  renderWorkSpaceLeft = function() {
    const pathname = location.pathname
    let isShow = false
    if (/terminal|workspace|tensorboard|dataset/.test(pathname)) {
      isShow = true
    }
    return (
      <WorkspaceLeft key={pathname} isShow={isShow}/>
    )
  }

  useImperativeHandle(
    ref,
    () => {
      return {
        setRightLineSelectKey
      }
    },
    [],
  )

  useEffect(() => {
    if(isTraveler()){
      window.location.href = '/team'
    }
  }, [])



  return (
    <contentContext.Provider value={{
      setShowSaveVersion
    }}>

      {renderWorkSpaceLeft()}
      <RouterConfig />

      {showRightLine() ? (
        <Fragment>
          <RightSiderShow rightLineSelectKey={rightLineSelectKey} />
          <RightSideLine
            showVersionDrawer={showVersionDrawer}
            setShowVersionDrawer={setShowVersionDrawerWithOther}
            rightLineSelectKey={rightLineSelectKey}
            setRightLineSelectKey={setRightLineSelectKeyWithOther}
          />
        </Fragment>
      ) : null}

      <RenderVersionPanel
        activeTabKey={activeTabPath}
        showSaveVersion={showSaveVersion}
        setShowSaveVersion={setShowSaveVersion}
        showVersionDrawer={showVersionDrawer}
        setShowVersionDrawer={setShowVersionDrawerWithOther}
      />

      <DecisionClassification/>


    </contentContext.Provider>
  )
}

export default React.forwardRef(Content)
